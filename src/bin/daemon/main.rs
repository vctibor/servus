#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::net::{TcpStream};
use std::env;
use std::time::Duration;
use std::collections::HashMap;
use ssh2::Session;
use servus::entity::Job as JobEntity;
use servus::entity::{AnyError};
use servus::entity::TxLog as LogEntry;
use servus::persistence::write_log;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use tokio::time::delay_for;
use job_scheduler::{JobScheduler, Job};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use uuid::Uuid;
use ::r2d2::Pool;

/// Number of milliseconds to sleep between every job scheduler check.
/// Perhaps should be configurable.
const REFRESH_RATE: u64 = 500;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref POOL: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Failed to read env var DATABASE_URL.");
    
        let manager = ConnectionManager::<PgConnection>::new(database_url);
    
        r2d2::Pool::builder().build(manager).expect("Failed to build connection pool.")
    };
}

/// Represents scheduled job
struct ScheduledJob {
    scheduled_job_id: Uuid,

    /// Last update of DEFINITION of job.
    last_update: NaiveDateTime
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
fn exec_remote(username: &str, url: &str, port: i32, command: &str)
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

fn write_log_success(stdout: &str, msg: &str, job_id: Uuid) {
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

    write_log_entry(entry);
}

fn write_log_err(stderr: &str, msg: &str, job_id: Uuid) {
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

    write_log_entry(entry);
}

fn write_log_entry(log_entry: LogEntry) {
    
    let conn = POOL.get().unwrap();
    
    let write_result = write_log(log_entry, &conn);

    if write_result.is_err() {
        println!("Failed to write to log.");
    }
}

#[tokio::main]
async fn main() -> Result<(), AnyError>
{
    let mut job_scheduler = JobScheduler::new();

    let mut scheduled_jobs: HashMap<Uuid, ScheduledJob> = HashMap::new();

    println!("Started daemon.");

    loop {
        
        let conn = POOL.get().expect("Failed to obtain connection from pool.");

        let jobs = servus::persistence::get_jobs(&conn)?;
        
        for job in jobs {

            if let Some(job_id) = job.id {

                let (should_schedule, scheduled_id) = should_schedule_job(&job, &scheduled_jobs);

                if should_schedule {
    
                    println!("Attempt to schedule job {} ({}).", job.name, job_id);
    
                    if let Some(scheduled_id) = scheduled_id {
                        println!("Removing old instance {} of job {} ({}).", scheduled_id, job.name, job_id);
                        job_scheduler.remove(scheduled_id);
                        println!("Removed {}.", scheduled_id);
                    }
                    
                    let schedule = job.schedule.parse();
    
                    if schedule.is_err() {
                        println!("Failed to parse schedule {} for job {} ({}).", job.schedule, job.name, job_id);

                        let entry = LogEntry {
                            id: None,
                            stdout: None,
                            stderr: None,
                            success: false,
                            time: Local::now().naive_local(),
                            message: format!("Failed to parse schedule {} for job {} ({}).", job.schedule, job.name, job_id),
                            job: job_id,
                            job_name: None
                        };

                        let write_result = write_log(entry, &conn);
                        
                        if write_result.is_err() {
                            println!("Failed to write to log.");
                        }

                        continue;
                    }
    
                    let schedule = schedule.unwrap();
                    let username = job.target.username.clone();
                    let url = job.target.url.clone();
                    let port = job.target.port;
                    let command = job.code.clone();
                    let job_name = job.name.clone();

                    let scheduled_job_id = job_scheduler.add(Job::new(schedule, move || {
                        
                        let execution_res = exec_remote(&username, &url, port, &command);
                        
                        match execution_res {
                            Ok(stdout) => {
                                let msg = format!("Succesfully executed job {} ({}).", job_name.clone(), job_id);
                                println!("{}", msg.clone());
                                write_log_success(&stdout, &msg, job_id);
                            }
                            
                            Err(stderr) => {
                                let msg = format!("Execution of job {} ({}) failed: {}.", job_name.clone(), job_id, stderr);
                                println!("{}", msg.clone());
                                write_log_err(&stderr.to_string(), &msg, job_id);
                            }
                        }
                    }));
    
                    scheduled_jobs.insert(job.id.unwrap(), ScheduledJob {
                        scheduled_job_id,
                        last_update: job.last_update.unwrap()
                    });

                    println!("Successfully scheduled job {} ({}) with ID {}.", job.name, job_id, scheduled_job_id);
                }
            }
        }

        job_scheduler.tick();

        delay_for(Duration::from_millis(REFRESH_RATE)).await;
    }
}