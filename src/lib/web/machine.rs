use crate::persistence::*;
use crate::entity::Machine as MachineEntity;
use crate::DbPool;
use uuid::Uuid;
use actix_web::{web, Error, HttpResponse};

pub async fn list_machines(pool: web::Data<DbPool>)
                    -> Result<HttpResponse, Error>
{
    // println!("List machines.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let machines = web::block(move || machine::get_machines(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // println!("{:?}", machines);

    Ok(HttpResponse::Ok().json(machines))
}

pub async fn get_machine(machine_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Get machine.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    let machine = web::block(move || machine::get_machine(machine_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(machine))
}

pub async fn create_machine(machine: web::Json<MachineEntity>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Create machine {:?}", machine);
    
    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let machine = machine::add_machine(machine.into_inner(), &conn)
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(machine))
}

pub async fn update_machine(machine_id: web::Path<Uuid>,
                     machine: web::Json<MachineEntity>,
                     pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Update machine {:?}", machine);

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || machine::update_machine(machine.into_inner(),
                            machine_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_machines(machines: web::Json<Vec<MachineEntity>>,
                             pool: web::Data<DbPool>)
                             -> Result<HttpResponse, Error>
{
    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || machine::update_machines(machines.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_machine(machine_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Delete machine.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || machine::delete_machine(machine_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    Ok(HttpResponse::Ok().finish())
}