use std::io::prelude::*;
use std::net::{TcpStream};
use std::env;
use std::time::Duration;
use std::collections::HashMap;
use ssh2::Session;
use servus::entity::Job as JobEntity;
use servus::entity::{AnyError};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use tokio::time::delay_for;
use job_scheduler::{JobScheduler, Job};
use chrono::NaiveDateTime;
use uuid::Uuid;

/// Number of milliseconds to sleep between every job scheduler check.
/// Perhaps should be configurable.
const REFRESH_RATE: u64 = 500;

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

    return Ok(output);
}

/// Should check if job was already scheduled, and compare update dates.
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

#[tokio::main]
async fn main() -> Result<(), AnyError>
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder().build(manager)?;
        
    let mut job_scheduler = JobScheduler::new();
      

    let mut scheduled_jobs: HashMap<Uuid, ScheduledJob> = HashMap::new();

    println!("Started daemon.");

    loop {
        
        let conn = pool.get().expect("Failed to obtain connection from pool.");

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

                        // TODO: log to DB

                        continue;
                    }
    
                    let schedule = schedule.unwrap();
                    
                    // We will have to keep map of <job_id, scheduled_job_id>,
                    //   and rescheduled every updated job.
                    let scheduled_job_id = job_scheduler.add(Job::new(schedule, || {
                        
                        let username = "malky";
                        let url = "dockerhost.malkynet";
                        let port = 22;
                        let command = "ls -lh";

                        let execution_res = exec_remote(username, url, port, command);
                        
                        println!("{:?}", execution_res);

                        // TODO: log to DB
                    }));
    
                    scheduled_jobs.insert(job.id.unwrap(), ScheduledJob {
                        scheduled_job_id: scheduled_job_id,
                        last_update: job.last_update.unwrap().clone()
                    });

                    println!("Successfully scheduled job {} ({}) with ID {}.", job.name, job_id, scheduled_job_id);
                }
            }
        }

        job_scheduler.tick();

        delay_for(Duration::from_millis(REFRESH_RATE)).await;
    }
}