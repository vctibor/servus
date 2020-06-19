#[macro_use]
extern crate diesel_migrations;

use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware};
use actix_web::web::{scope, resource, get, post};
use actix_web_static_files;
use actix_web_static_files::ResourceFiles;
use servus::web::*;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use servus::execution::*;
use diesel_migrations::*;
use actix_web::HttpResponse;
use actix_http::http;


embed_migrations!("migrations");

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Number of milliseconds to sleep between every job scheduler check.
const REFRESH_RATE: u64 = 500;

async fn redirect_to_index() -> HttpResponse {
    HttpResponse::PermanentRedirect().header(http::header::LOCATION, "/").finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    {
        use ssh2::Session;

        // Almost all APIs require a `Session` to be available
        let sess = Session::new().unwrap();
        let mut agent = sess.agent().unwrap();
    
        // Connect the agent and request a list of identities
        agent.connect().unwrap();
        agent.list_identities().unwrap();
    
        for identity in agent.identities().unwrap() {
            println!("{}", identity.comment());
            let pubkey = identity.blob();
        }
    }

    
    println!("hello world");

    // servus::execution::start_ssh_agent()?;

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();

    let database_url = env::var("SERVUS_DATABASE_URL")
        .expect("SERVUS_DATABASE_URL must be set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let conn = pool.get().expect("Failed to obtain connection from pool.");

    embedded_migrations::run(&conn)
        .expect("Failed to run migrations to current version of database schema.");

    let bind = env::var("SERVUS_LISTEN_ON")
        .expect("SERVUS_LISTEN_ON must be set.");

    let daemon_pool = pool.clone();


    thread::spawn(|| {
        
        let mut job_scheduler = ServusJobScheduler::new(daemon_pool);

        println!("Started daemon.");
    
        loop {
            
            job_scheduler.schedule_jobs();
    
            job_scheduler.tick();
    
            thread::sleep(Duration::from_millis(REFRESH_RATE));
        }

    });

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        
        // load embedded static files
        let generated = generate();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
            .data(pool.clone())
            .service(
                scope("/api")
                    .service(scope("/job")
                        .service(resource("/list").route(get().to(job::list_jobs)))
                        .service(resource("/get/{job_id}").route(get().to(job::get_job)))
                        .service(resource("/create").route(post().to(job::create_job)))
                        .service(resource("/update/{job_id}").route(post().to(job::update_job)))
                        .service(resource("/bulk_update").route(post().to(job::update_jobs)))
                        .service(resource("/delete/{job_id}").route(post().to(job::delete_job)))
                        .service(resource("/exec/{job_id}").route(get().to(job::execute))))
                    .service(scope("/machine")
                        .service(resource("/list").route(get().to(machine::list_machines)))
                        .service(resource("/get/{machine_id}").route(get().to(machine::get_machine)))
                        .service(resource("/create").route(post().to(machine::create_machine)))
                        .service(resource("/update/{machine_id}").route(post().to(machine::update_machine)))
                        .service(resource("/bulk_update").route(post().to(machine::update_machines)))
                        .service(resource("/delete/{machine_id}").route(post().to(machine::delete_machine))))
                    .service(scope("/user")
                        .service(resource("/list").route(get().to(user::list_users)))
                        .service(resource("/get/{user_id}").route(get().to(user::get_user)))
                        .service(resource("/create").route(post().to(user::create_user)))
                        .service(resource("/update/{user_id}").route(post().to(user::update_user)))
                        .service(resource("/bulk_update").route(post().to(user::update_users)))
                        .service(resource("/delete/{user_id}").route(post().to(user::delete_user))))
                    .service(scope("/log")
                        .service(resource("{offset}/{entries}").route(get().to(log::get_log_entries))))
                    
            )
            .service(resource("/jobs").route(get().to(redirect_to_index)))
            .service(resource("/jobs/").route(get().to(redirect_to_index)))
            .service(resource("/machines").route(get().to(redirect_to_index)))
            .service(resource("/machines/").route(get().to(redirect_to_index)))
            .service(resource("/users").route(get().to(redirect_to_index)))
            .service(resource("/users/").route(get().to(redirect_to_index)))
            .service(resource("/log").route(get().to(redirect_to_index)))
            .service(resource("/log/").route(get().to(redirect_to_index)))
            .service(ResourceFiles::new("/", generated))
        })
        .bind(&bind)?
        .run()
        .await
}