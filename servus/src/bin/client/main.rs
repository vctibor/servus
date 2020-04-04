mod user;
mod machine;

use actix_web::{web, middleware, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use actix_files as fs;

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
            .wrap(middleware::Logger::default())
            .data(pool.clone())

            .service(web::resource("/api/user/list").route(web::get().to(user::list_users)))
            .service(web::resource("/api/user/get/{user_id}").route(web::get().to(user::get_user)))
            .service(web::resource("/api/user/create").route(web::post().to(user::create_user)))
            .service(web::resource("/api/user/update/{user_id}").route(web::post().to(user::update_user)))
            .service(web::resource("/api/user/delete/{user_id}").route(web::post().to(user::delete_user)))

            .service(web::resource("/api/machine/list").route(web::get().to(machine::list_machines)))
            .service(web::resource("/api/machine/get/{machine_id}").route(web::get().to(machine::get_machine)))
            .service(web::resource("/api/machine/create").route(web::post().to(machine::create_machine)))
            .service(web::resource("/api/machine/update/{machine_id}").route(web::post().to(machine::update_machine)))
            .service(web::resource("/api/machine/delete/{machine_id}").route(web::post().to(machine::delete_machine)))
            
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
        .bind(&bind)?
        .run()
        .await
}