use crate::persistence::*;
use crate::entity::Job as JobEntity;
use crate::DbPool;
use crate::execution::exec_remote;
use uuid::Uuid;
use actix_web::{web, Error, HttpResponse};

pub async fn list_jobs(pool: web::Data<DbPool>)
                    -> Result<HttpResponse, Error>
{
    // println!("List jobs.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let jobs = web::block(move || job::get_jobs(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    // println!("{:?}", jobs);

    Ok(HttpResponse::Ok().json(jobs))
}

pub async fn get_job(job_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Get job.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    let job = web::block(move || job::get_job(job_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(job))
}

pub async fn create_job(job: web::Json<JobEntity>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Create job {:?}", job);
    
    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let job = job::add_job(job.into_inner(), &conn)
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(job))
}

pub async fn update_job(job_id: web::Path<Uuid>,
                     job: web::Json<JobEntity>,
                     pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Update job {:?}", job);

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || job::update_job(job.into_inner(),
                            job_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_jobs(jobs: web::Json<Vec<JobEntity>>,
                         pool: web::Data<DbPool>)
                         -> Result<HttpResponse, Error>
{
    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || job::update_jobs(jobs.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_job(job_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                     -> Result<HttpResponse, Error>
{
    println!("Delete job.");

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    web::block(move || job::delete_job(job_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    Ok(HttpResponse::Ok().finish())
}


pub async fn execute(job_uid: web::Path<Uuid>, pool: web::Data<DbPool>)
                  -> Result<HttpResponse, Error>
{
    println!("Execute job {}", job_uid);

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    let job = web::block(move || job::get_job(job_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?
        .ok_or_else(|| HttpResponse::InternalServerError().finish())?;

    let username = job.target.username.clone();
    let url = job.target.url.clone();
    let port = job.target.port;
    let command = job.code.clone();
    //let job_name = job.name.clone();

    let execution_res = exec_remote(&username, &url, port, &command);

    println!("Execution result {:?}", execution_res);

    Ok(HttpResponse::Ok().finish())
}