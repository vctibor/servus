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
    pub last_update: Option<NaiveDateTime>,
    pub send_email: bool,
    pub execute_now: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewJob {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub schedule: String,
    pub target: Uuid,
    pub owner: Uuid,
    pub last_update: Option<NaiveDateTime>,
    pub send_email: bool,
    pub execute_now: bool
}

pub fn add_job(job: JobEntity, conn: &PgConnection)
                -> Result<JobEntity, AnyError>
{
    let target_id = job.target.id.ok_or_else(|| ServusError::new("Target ID not provided."))?;
    let owner_id = job.owner.id.ok_or_else(|| ServusError::new("Owner ID not provided."))?;

    let last_update_dt = Local::now().naive_local();

    let job = Job {
        id: Uuid::new_v4(),
        name: job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: target_id,
        owner: owner_id,
        last_update: Some(last_update_dt),
        send_email: job.send_email,
        execute_now: false
    };

    diesel::insert_into(jobs)
        .values(&job)
        .execute(conn)?;

    let target_entity = super::machine::get_machine(job.target, &conn)?
        .ok_or_else(|| ServusError::new("Invalid target ID."))?;

    let owner_entity = super::user::get_user(job.owner, &conn)?
        .ok_or_else(|| ServusError::new("Invalid owner ID."))?;
        
    Ok(JobEntity {
        id: Some(job.id),
        name: job.name,
        code: job.code.to_owned(),
        description: job.description.to_owned(),
        schedule: job.schedule.to_owned(),
        target: target_entity,
        owner: owner_entity,
        last_update: job.last_update,
        send_email: job.send_email,
        last_status: None
    })
}

pub fn get_jobs(conn: &PgConnection)
                -> Result<Vec<JobEntity>, AnyError>
{
    let job_table: Vec<Job> = jobs.order(name).load::<Job>(conn)?;

    let mut entities = Vec::with_capacity(job_table.len());

    for job in job_table.iter() {
            
        let target_entity = super::machine::get_machine(job.target, &conn)?
            .ok_or_else(|| ServusError::new("Invalid target ID."))?;

        let owner_entity = super::user::get_user(job.owner, &conn)?
            .ok_or_else(|| ServusError::new("Invalid owner ID."))?;            

        entities.push(JobEntity {
            id: Some(job.id),
            name: job.name.to_owned(),
            code: job.code.to_owned(),
            description: job.description.to_owned(),
            schedule: job.schedule.to_owned(),
            target: target_entity,
            owner: owner_entity,
            last_update: job.last_update,
            send_email: job.send_email,

            // TODO: check latest log entry
            last_status: Some(false)
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
                .ok_or_else(|| ServusError::new("Invalid target ID."))?;
    
            let owner_entity = super::user::get_user(job.owner, &conn)?
                .ok_or_else(|| ServusError::new("Invalid owner ID."))?;
            
            Ok(Some(JobEntity {
                id: Some(job.id),
                name: job.name,
                code: job.code,
                description: job.description,
                schedule: job.schedule,
                target: target_entity,
                owner: owner_entity,
                last_update: job.last_update,
                send_email: job.send_email,

                // TODO: check latest log entry
                last_status: Some(false)
            }))
        },
        None => Ok(None),
    }
}

pub fn update_job(job: JobEntity, job_id: Uuid, conn: &PgConnection)
                   -> Result<usize, AnyError>
{
    let target_id = job.target.id.ok_or_else(|| ServusError::new("Target ID not provided."))?;
    let owner_id = job.owner.id.ok_or_else(|| ServusError::new("Owner ID not provided."))?;

    let last_update_dt = Local::now().naive_local();

    let job = Job {
        id: job_id,
        name: job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: target_id,
        owner: owner_id,
        last_update: Some(last_update_dt),
        send_email: job.send_email,
        execute_now: false
    };

    let res = diesel::update(jobs::table)
        .filter(id.eq(job_id))    
        .set(&job).execute(conn)?;
        
    Ok(res)
}

pub fn update_jobs(mut updated_jobs: Vec<JobEntity>, conn: &PgConnection)
                  -> Result<(), AnyError>
{    
    let old_jobs: Vec<JobEntity> = get_jobs(&conn)?;
    let old_jobs_ids: Vec<Uuid> = old_jobs.into_iter().map(|job| job.id.unwrap()).rev().collect();

    let mut jobs_to_delete = old_jobs_ids.clone();

    while let Some(updated_job) = updated_jobs.pop() {
        if updated_job.id.is_none() || updated_job.id == Some(Uuid::nil())
        {
            add_job(updated_job, &conn)?;
        }
        else if let Some(updated_job_id) = updated_job.id
        {
            if old_jobs_ids.contains(&updated_job_id) {
                // update_job(updated_job, updated_job_id, &conn)?;
            } else {
                add_job(updated_job, &conn)?;
            }

            jobs_to_delete.retain(|&item| item != updated_job_id);
        }
    }

    for delete_id in jobs_to_delete {
        delete_job(delete_id, &conn)?;
    }

    Ok(())
}

pub fn delete_job(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    let res = diesel::delete(jobs.filter(id.eq(uid))).execute(conn)?;
    Ok(res)
}