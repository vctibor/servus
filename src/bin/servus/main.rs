#[macro_use]
extern crate diesel_migrations;

use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware};
use actix_web::web::{scope, resource, get, post};
use actix_files as fs;
use servus::web::*;
use std::thread;
use std::time::Duration;
use servus::execution::*;
use diesel_migrations::*;

embed_migrations!("migrations");

/// Number of milliseconds to sleep between every job scheduler check.
const REFRESH_RATE: u64 = 500;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

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
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(
                scope("/api")
                    .service(scope("/job")
                        .service(resource("/list").route(get().to(job::list_jobs)))
                        .service(resource("/get/{job_id}").route(get().to(job::get_job)))
                        .service(resource("/create").route(post().to(job::create_job)))
                        .service(resource("/update/{job_id}").route(post().to(job::update_job)))
                        .service(resource("/bulk_update").route(post().to(job::update_jobs)))
                        .service(resource("/delete/{job_id}").route(post().to(job::delete_job))))
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
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
        })
        .bind(&bind)?
        .run()
        .await
}