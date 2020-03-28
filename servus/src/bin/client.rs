use servus::persistence::*;

use actix_web::{web, middleware, App, Error, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use actix_files as fs;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn list_users(pool: web::Data<DbPool>)
                    -> Result<HttpResponse, Error>
{
    println!("List users.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let users = user::get_users(&conn);
    println!("{:?}", users);
    Ok(HttpResponse::Ok().json(users))
}

async fn get_user(user_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Get user.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = user::get_user(user_uid.into_inner(), &conn);
    Ok(HttpResponse::Ok().json(user))
}

async fn create_user(user: web::Json<user::NewUser>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Create user.");
    println!("{:?}", user);
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = user::add_user(user.into_inner(), &conn).unwrap();
    Ok(HttpResponse::Ok().json(user))
}

async fn update_user(user_uid: web::Path<Uuid>, user: web::Json<user::NewUser>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Update user.");
    println!("{:?}", user);
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = user.into_inner();
    let user = user::User { id: user_uid.into_inner(), name: user.name, email: user.email };

    user::update_user(user, &conn);
    Ok(HttpResponse::Ok().finish())
}

async fn delete_user(user_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Delete user.");
    let conn = pool.get().expect("couldn't get db connection from pool");
    user::delete_user(user_uid.into_inner(), &conn);
    Ok(HttpResponse::Ok().finish())
}

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
            //.data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/api/user/list").route(web::get().to(list_users)))
            .service(web::resource("/api/user/get/{user_id}").route(web::get().to(get_user)))
            .service(web::resource("/api/user/create").route(web::post().to(create_user)))
            .service(web::resource("/api/user/update/{user_id}").route(web::post().to(update_user)))
            .service(web::resource("/api/user/delete/{user_id}").route(web::post().to(delete_user)))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
        .bind(&bind)?
        .run()
        .await
}