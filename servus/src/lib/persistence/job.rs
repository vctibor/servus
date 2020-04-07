use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entity::Job as JobEntity;
use crate::entity::{AnyError, ServusError};
use crate::schema::jobs;
use crate::schema::jobs::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
struct Job {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub schedule: String,
    pub target: Uuid,
    pub owner: Uuid,
    pub last_update: NaiveDateTime,
    pub send_email: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewJob {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub schedule: String,
    pub target: Uuid,
    pub owner: Uuid,
    pub last_update: NaiveDateTime,
    pub send_email: bool
}

pub fn add_job(job: JobEntity, conn: &PgConnection)
                -> Result<JobEntity, AnyError>
{
    let target_id = job.target.id.ok_or(ServusError::new("Target ID not provided."))?;
    let owner_id = job.owner.id.ok_or(ServusError::new("Owner ID not provided."))?;

    let last_update_dt = Local::now().naive_local();

    let job = Job {
        id: Uuid::new_v4(),
        name: job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: target_id,
        owner: owner_id,
        last_update: last_update_dt,
        send_email: job.send_email
    };

    diesel::insert_into(jobs)
        .values(&job)
        .execute(conn)?;

    let target_entity = super::machine::get_machine(job.target, &conn)?
        .ok_or(ServusError::new("Invalid target ID."))?;

    let owner_entity = super::user::get_user(job.owner, &conn)?
        .ok_or(ServusError::new("Invalid owner ID."))?;
        
    Ok(JobEntity {
        id: Some(job.id),
        name: job.name,
        code: job.code.to_owned(),
        description: job.description.to_owned(),
        schedule: job.schedule.to_owned(),
        target: target_entity,
        owner: owner_entity,
        last_update: job.last_update,
        send_email: job.send_email
    })
}

pub fn get_jobs(conn: &PgConnection)
                -> Result<Vec<JobEntity>, AnyError>
{
    let job_table: Vec<Job> = jobs.load::<Job>(conn)?;

    let mut entities = Vec::with_capacity(job_table.len());

    for job in job_table.iter() {
            
        let target_entity = super::machine::get_machine(job.target, &conn)?
            .ok_or(ServusError::new("Invalid target ID."))?;

        let owner_entity = super::user::get_user(job.owner, &conn)?
            .ok_or(ServusError::new("Invalid owner ID."))?;            

        entities.push(JobEntity {
            id: Some(job.id),
            name: job.name.to_owned(),
            code: job.code.to_owned(),
            description: job.description.to_owned(),
            schedule: job.schedule.to_owned(),
            target: target_entity,
            owner: owner_entity,
            last_update: job.last_update,
            send_email: job.send_email
        });
    }
    
    Ok(entities)
}

pub fn get_job(uid: Uuid, conn: &PgConnection)
                -> Result<Option<JobEntity>, AnyError>
{
    let job = jobs
        .filter(id.eq(uid))
        .first::<Job>(conn)
        .optional()?;

    match job {
        Some(job) => {

            let target_entity = super::machine::get_machine(job.target, &conn)?
                .ok_or(ServusError::new("Invalid target ID."))?;
    
            let owner_entity = super::user::get_user(job.owner, &conn)?
                .ok_or(ServusError::new("Invalid owner ID."))?;
            
            Ok(Some(JobEntity {
                id: Some(job.id),
                name: job.name,
                code: job.code,
                description: job.description,
                schedule: job.schedule,
                target: target_entity,
                owner: owner_entity,
                last_update: job.last_update,
                send_email: job.send_email
            }))
        },
        None => Ok(None),
    }
}

pub fn update_job(job: JobEntity, job_id: Uuid, conn: &PgConnection)
                   -> Result<usize, AnyError>
{
    let target_id = job.target.id.ok_or(ServusError::new("Target ID not provided."))?;
    let owner_id = job.owner.id.ok_or(ServusError::new("Owner ID not provided."))?;

    let last_update_dt = Local::now().naive_local();

    let job = Job {
        id: job_id,
        name: job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: target_id,
        owner: owner_id,
        last_update: last_update_dt,
        send_email: job.send_email
    };

    let res = diesel::update(jobs::table).set(&job).execute(conn)?;
    Ok(res)
}

pub fn delete_job(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    let res = diesel::delete(jobs.filter(id.eq(uid))).execute(conn)?;
    Ok(res)
}