mod schema;
mod ssh;
mod web;
mod data_access_layer;

use ssh::exec_remote;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use tokio::time::delay_for;
use tokio::task;

use web::Web;

use crate::data_access_layer::tx_log::LogEntry;
use crate::data_access_layer::target::Target;
use crate::data_access_layer::user::User;

use crate::data_access_layer::*;


use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::data_access_layer::user::*;


#[macro_use]
extern crate diesel;
extern crate dotenv;


use gotham::state::State;

const HELLO_WORLD: &'static str = "Hello World!";

/// Function to create Diesel connection to Postgres DB.
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    let conn = establish_connection();

    (state, HELLO_WORLD)
}

extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::{Body, Response, StatusCode};
use gotham::helpers::http::response::create_response;

fn get_users(mut state: State) -> (State, Response<Body>) {
    let res = {
        create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_vec("sdf").expect("serialized product"),
        )
    };
    (state, res)
}

#[tokio::main]
async fn main() {

    let server = task::spawn(async {
        /*
        let w = Web::new();
        w.serve().await;
        */

        let addr = "127.0.0.1:7878";
        println!("Listening for requests at http://{}", addr);
        gotham::start(addr, || Ok(say_hello))
    });

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