use servus::persistence::*;
use servus::entity::User as UserEntity;
use uuid::Uuid;
use actix_web::{web, Error, HttpResponse};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn list_users(pool: web::Data<DbPool>)
                    -> Result<HttpResponse, Error>
{
    println!("List users.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let users = web::block(move || user::get_users(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("{:?}", users);

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(user_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Get user.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    let user = web::block(move || user::get_user(user_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(user: web::Json<UserEntity>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Create user {:?}", user);
    
    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let user = user::add_user(user.into_inner(), &conn)
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(user_id: web::Path<Uuid>,
                     user: web::Json<UserEntity>,
                     pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Update user {:?}", user);

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || user::update_user(user.into_inner(),
                            user_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_user(user_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Delete user.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || user::delete_user(user_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    Ok(HttpResponse::Ok().finish())
}