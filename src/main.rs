#[macro_use]
extern crate diesel;

mod model;
mod schema;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn list_users(pool: web::Data<DbPool>)
    -> Result<HttpResponse, Error>
{
    println!("List users.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let users = model::user::get_users(&conn);
    println!("{:?}", users);
    Ok(HttpResponse::Ok().json(users))
}

async fn get_user(user_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
    -> Result<HttpResponse, Error>
{
    println!("Get user.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = model::user::get_user(user_uid.into_inner(), &conn);
    Ok(HttpResponse::Ok().json(user))
}

async fn create_user(user: web::Json<model::user::NewUser>, pool: web::Data<DbPool>)
    -> Result<HttpResponse, Error>
{
    println!("Create user.");
    println!("{:?}", user);
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = model::user::add_user(user.into_inner(), &conn).unwrap();
    Ok(HttpResponse::Ok().json(user))
}

async fn update_user(user_uid: web::Path<Uuid>, user: web::Json<model::user::NewUser>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Update user.");
    println!("{:?}", user);
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = user.into_inner();
    let user = model::user::User { id: user_uid.into_inner(), name: user.name, email: user.email };

    model::user::update_user(user, &conn);
    Ok(HttpResponse::Ok().finish())
}

async fn delete_user(user_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Delete user.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    model::user::delete_user(user_uid.into_inner(), &conn);
    Ok(HttpResponse::Ok().finish())
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
            //.data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/api/user/list").route(web::get().to(list_users)))
            .service(web::resource("/api/user/get/{user_id}").route(web::get().to(get_user)))
            .service(web::resource("/api/user/create").route(web::post().to(create_user)))
            .service(web::resource("/api/user/update/{user_id}").route(web::post().to(update_user)))
            .service(web::resource("/api/user/delete/{user_id}").route(web::post().to(delete_user)))
    })
        .bind(&bind)?
        .run()
        .await
}