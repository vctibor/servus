use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

use servus::entity::Job as JobEntity;
use servus::entity::{AnyError, ServusError};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

use tokio::time::delay_for;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;



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

pub fn exec_remote_job(job: JobEntity) -> Result<String, AnyError> {
    let username = job.target.username;
    let url = job.target.url;
    let port = job.target.port;
    let command = job.code;
    exec_remote(&username, &url, port, &command)
}

#[tokio::main]
async fn main() {
    
    // while true:
    //     for every job:
    //         - compare last_update in memory vs. last_update in db
    //         - reschedule jobs that were updated since last scheduling
    // 
    //     check if any job is scheduled for this time
    //     if yes:
    //         - get job details
    //         - execute job
    //         - log stderr, stdout, exit status
    
    println!("This is daemon.");


    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let conn = pool.get().expect("Failed to obtain connection from pool.");

    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/3 * * * * *".parse().unwrap(), || {
        /*
        let username = "malky";
        let url = "dockerhost.malkynet";
        let port = 22;
        let command = "ls -lh";
        let res = exec_remote(username, url, port, command);
        println!("{}", res)
        */
        println!("test");
    }));

    loop {
        sched.tick();
        delay_for(Duration::from_millis(500)).await;
    }
    
}