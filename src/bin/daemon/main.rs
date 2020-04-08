use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

use servus::entity::Job as JobEntity;
use servus::entity::{AnyError};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

use tokio::time::delay_for;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;


/// Number of milliseconds to sleep between every job scheduler check.
/// Perhaps should be configurable.
const REFRESH_RATE: u64 = 500;


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

    return Ok(output);
}

pub fn exec_remote_job(job: &JobEntity) -> Result<String, AnyError> {
    let username = job.target.username.clone();
    let url = job.target.url.clone();
    let port = job.target.port;
    let command = job.code.clone();
    exec_remote(&username, &url, port, &command)
}

#[tokio::main]
async fn main() -> Result<(), AnyError>
{
    //
    //  let scheduled_jobs = Map<job_id, scheduled_job_id>   // have map structure mapping job "type" to scheduled "instance"
    //
    //  while true:
    //
    //      let jobs = servus::persistence::get_jobs()           // obtain list of all scheduled jobs
    // 
    //      if job not in scheduled_jobs:
    //          -- get job details
    //          -- schedule job
    //      elif job.last_update > scheduled_job.last_update:
    //          -- remove old instance
    //          -- reschedue job
    //      else:
    //          -- skip
    //  
    //  job_scheduler.tick()
    //      -- log stdout, stderr, exit status


    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder().build(manager)?;
        
    let mut job_scheduler = JobScheduler::new();
        
    loop {
        
        let conn = pool.get().expect("Failed to obtain connection from pool.");

        let jobs = servus::persistence::get_jobs(&conn)?;

        for job in jobs {

            let schedule = "* 1/3 * * * *".parse();

            if schedule.is_err() {
                println!("Failed to parse schedule for job {}", job.id.unwrap());
                continue;
            }

            let schedule = schedule.unwrap();
            
            // We will have to keep map of <job_id, scheduled_job_id>,
            //   and rescheduled every updated job.
            let scheduled_job_id = job_scheduler.add(Job::new(schedule, || {
                
                /*
                let username = "malky";
                let url = "dockerhost.malkynet";
                let port = 22;
                let command = "ls -lh";
                let res = exec_remote(username, url, port, command);
                println!("{}", res)
                */

                let exec_res = exec_remote_job(&job);

            }));
        }

        job_scheduler.tick();

        delay_for(Duration::from_millis(REFRESH_RATE)).await;
    }
    
}