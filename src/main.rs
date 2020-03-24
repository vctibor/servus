#[macro_use]
extern crate diesel;

mod model;
mod schema;

use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Serialize, Deserialize};

use crate::model::user;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/get_users")]
async fn get_users(pool: web::Data<DbPool>)
    -> Result<HttpResponse, Error>
{
    println!("Get users.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let users = user::get_users(&conn);
    println!("{:?}", users);
    Ok(HttpResponse::Ok().json(users))
}

async fn add_user_vcff(item: web::Json<user::NewUser>, pool: web::Data<DbPool>) -> HttpResponse
{
    println!("Add user");

    println!("{:?}", item);

    let conn = pool.get().expect("couldn't get db connection from pool");

    user::add_user(item.into_inner(), &conn);

    /*


    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || user::add_user(payload.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
    */

    HttpResponse::Ok().json("added user")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

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
            .data(web::JsonConfig::default().limit(4096))
            // .service(add_user)
            .service(web::resource("/extractor").route(web::post().to(add_user_vcff)))
            .service(get_users)
            // .service(web::resource("/extractor").route(web::post().to(index)))
    })
        .bind(&bind)?
        .run()
        .await

}