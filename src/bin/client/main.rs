use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use actix_web::{App, HttpServer};
use actix_web::web::{scope, resource, get, post};
use actix_files as fs;

use servus::web::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
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