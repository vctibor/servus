use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

use crate::data_access_layer::job::Job;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

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

pub fn exec_remote_job(job: Job) -> Result<String, AnyError> {
    let username = job.target.username;
    let url = job.target.url;
    let port = job.target.port;
    let command = job.code;
    exec_remote(&username, &url, port, &command)
}