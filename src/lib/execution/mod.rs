use chrono::NaiveDateTime;
use uuid::Uuid;
use ssh2::Session;
use crate::entity::{AnyError, ServusError};
use crate::entity::Job as JobEntity;
use crate::entity::TxLog as LogEntry;
use crate::persistence::log::write_log;
use crate::persistence::{get_jobs, get_job};
use crate::DbPool;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;
use job_scheduler::{JobScheduler, Job};
use chrono::prelude::*;
use diesel::pg::PgConnection;

/// Represents scheduled job
struct ScheduledJob {
    scheduled_job_id: Uuid,

    /// Last update of DEFINITION of job.
    last_update: NaiveDateTime
}

pub struct ServusJobScheduler {

    pool: DbPool,

    job_scheduler: JobScheduler<'static>,

    /// Map between scheduled instances and definitions of jobs.
    scheduled_jobs: HashMap<Uuid, ScheduledJob>
}

impl ServusJobScheduler {

    pub fn new(pool: DbPool) -> ServusJobScheduler {
        ServusJobScheduler {
            pool,
            job_scheduler: JobScheduler::new(),
            scheduled_jobs: HashMap::new()
        }
    }

    pub fn schedule_jobs(&mut self) -> Result<(), AnyError> {
        let conn = self.pool.get()?;
        
        // list of jobs defined in database
        let jobs: Vec<JobEntity> = get_jobs(&conn)?;
        
        let mut to_remove = vec!();
        for scheduled_job in &self.scheduled_jobs {
            if get_job(scheduled_job.0.clone(), &conn)?.is_none() {
                println!("Should remove job");
                to_remove.push(scheduled_job.0.clone());
            }
        }

        for job_to_remove in to_remove {
            let id = self.scheduled_jobs.get(&job_to_remove).unwrap().scheduled_job_id;
            if self.job_scheduler.remove(id) { 
                self.scheduled_jobs.remove_entry(&job_to_remove);
                println!("Removed job ID {}", &job_to_remove);
            } else {
                println!("Failed to remove job ID {}", &job_to_remove);
            }
        }

        /*
        let mut to_remove: Vec<Uuid> = vec!();

        for scheduled_job in &self.scheduled_jobs {
            let mut remove = true;
            for job in &jobs {
                if job.id.unwrap() == *scheduled_job.0 {
                    remove = false;
                }
            }

            if remove {
                to_remove.push(*scheduled_job.0);
            }
        }

        for job_to_remove in to_remove {
            println!("Removed job ID {}", &job_to_remove);
            self.scheduled_jobs.remove_entry(&job_to_remove);
            self.job_scheduler.remove(job_to_remove);
        }
        */

        for job in jobs {            
            let res = self.schedule_job(&job);
            if res.is_err() {
                println!("Failed to schedule job {}", job.name);
            }
        }

        Ok(())
    }

    pub fn schedule_job(&mut self, job: &JobEntity) -> Result<(), AnyError> {
        
        if let Some(job_id) = job.id {

            let (should_schedule, scheduled_id) = should_schedule_job(&job, &self.scheduled_jobs);

            if should_schedule {

                println!("Attempt to schedule job {} ({}).", job.name, job_id);

                if let Some(scheduled_id) = scheduled_id {
                    println!("Removing old instance {} of job {} ({}).", scheduled_id, job.name, job_id);
                    self.job_scheduler.remove(scheduled_id);
                    println!("Removed {}.", scheduled_id);
                }

                let schedule = job.schedule.parse().unwrap();

                let job_closure = self.job_to_closure(&job)?;
            
                let scheduled_job_id = self.job_scheduler.add(Job::new(schedule, job_closure));

                println!("Scheduled job {}", scheduled_job_id);

                self.scheduled_jobs.insert(job_id, ScheduledJob {
                    scheduled_job_id,
                    last_update: job.last_update.unwrap()
                });
            }
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        self.job_scheduler.tick();
    }

    // PRIVATE

    fn job_to_closure(&self, job: &JobEntity) -> Result<impl FnMut(), AnyError> {

        let conn = self.pool.get()?;
    
        let job_id = job.id.ok_or_else(|| {
            ServusError::new("Can't schedule job without ID")
        })?;
    
        let username = job.target.username.clone();
        let url = job.target.url.clone();
        let port = job.target.port;
        let command = job.code.clone();
        let job_name = job.name.clone();
    
        Ok(move || {
                            
            let execution_res = exec_remote(&username, &url, port, &command);
            
            match execution_res {
                Ok(stdout) => {
                    let msg = format!("Succesfully executed job {} ({}).", job_name.clone(), job_id);
                    println!("{}", msg.clone());
                    write_log_success(&stdout, &msg, job_id, &conn);
                }
                
                Err(stderr) => {
                    let msg = format!("Execution of job {} ({}) failed: {}.", job_name.clone(), job_id, stderr);
                    println!("{}", msg.clone());
                    write_log_err(&stderr.to_string(), &msg, job_id, &conn);
                }
            }
        })
    }
}

/// Executes provided command on remote machine using ssh.
/// Note that source machine has to have key-based access to target machine,
/// ssh-agent has to be configured and imported identity specified by 'username' parameter.
///
/// # start ssh agent:
/// > eval `ssh-agent`
///
/// # add ssh identities:
/// > ssh-add
///
/// # list ssh identities:
/// > ssh-add -l
/// 
/// This is performed by function `start_ssh_agent()`.
pub fn exec_remote(username: &str, url: &str, port: i32, command: &str)
    -> Result<String, AnyError>
{
    let addr = format!("{}:{}", url, port);

    let tcp = TcpStream::connect(addr)?;

    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_agent(username)?;

    let mut channel = session.channel_session()?;
    channel.exec(command)?;

    let mut output = String::new();

    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    channel.exit_status()?;

    Ok(output)
}

/// Checks if job was already scheduled and compares update dates.
/// Returns 'true' if job SHOULD be scheduled.
/// Returns scheduled job ID, if job was scheduled, but definition was updated,
/// and old scheduled instance should be removed. 
/// TODO: This could REALLY use some unit tests.
fn should_schedule_job(job: &JobEntity, scheduled_jobs: &HashMap<Uuid, ScheduledJob>) -> (bool, Option<Uuid>) {
    
    if job.id.is_none() || job.last_update.is_none() {
        return (true, None);
    }

    if let Some(scheduled_job) = scheduled_jobs.get(&job.id.unwrap()) {
        if job.last_update.unwrap() > scheduled_job.last_update {
            return (true, Some(scheduled_job.scheduled_job_id));
        }

        return (false, None);
    }

    (true, None)
}

fn write_log_success(stdout: &str, msg: &str, job_id: Uuid, conn: &PgConnection) {
    let entry = LogEntry {
        id: None,
        stdout: Some(stdout.to_owned()),
        stderr: None,
        success: true,
        time: Local::now().naive_local(),
        message: msg.to_owned(),
        job: job_id,
        job_name: None
    };

    write_log_entry(entry, &conn);
}

fn write_log_err(stderr: &str, msg: &str, job_id: Uuid, conn: &PgConnection) {
    let entry = LogEntry {
        id: None,
        stdout: None,
        stderr: Some(stderr.to_owned()),
        success: false,
        time: Local::now().naive_local(),
        message: msg.to_owned(),
        job: job_id,
        job_name: None
    };

    write_log_entry(entry, &conn);
}

fn write_log_entry(log_entry: LogEntry, conn: &PgConnection) {
    
    let write_result = write_log(log_entry, &conn);

    if write_result.is_err() {
        println!("Failed to write to log.");
    }
}