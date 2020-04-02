/*
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::user::User;
use super::machine::Machine;
use crate::entity::Job as JobEntity;
use crate::entity::User as UserEntity;
use crate::entity::Machine as MachineEntity;
use crate::schema::jobs;
use crate::schema::jobs::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;

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
                -> Result<JobEntity, diesel::result::Error>
{
    let new_job = Job {
        id: Uuid::new_v4(),
        name: job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: job.target,
        owner: job.owner,
        last_update: job.last_update,
        send_email: job.send_email
    };

    diesel::insert_into(jobs)
        .values(&new_job)
        .execute(conn)?;

    Ok(JobEntity {
        id: Some(new_job.id),
        name: new_job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: job.target,
        owner: job.owner,
        last_update: job.last_update,
        send_email: job.send_email
    })
}

pub fn get_jobs(conn: &PgConnection)
                 -> Result<Vec<JobEntity>, diesel::result::Error>
{
    let job_table: Vec<Job> = jobs.load::<Job>(conn)?;

    let mut entities = Vec::with_capacity(job_table.len());

    for job in job_table.iter() {
        entities.push(JobEntity {
            id: Some(job.id),
            name: job.name.to_owned(),
            code: job.code,
            description: job.description,
            schedule: job.schedule,
            target: job.target,
            owner: job.owner,
            last_update: job.last_update,
            send_email: job.send_email
        });
    }
    
    Ok(entities)
}

pub fn get_job(uid: Uuid, conn: &PgConnection)
                -> Result<Option<JobEntity>, diesel::result::Error>
{
    let job = jobs
        .filter(id.eq(uid))
        .first::<Job>(conn)
        .optional()?;

    match job {
        Some(job) => Ok(Some(JobEntity {
            id: Some(job.id),
            name: job.name,
            code: job.code,
            description: job.description,
            schedule: job.schedule,
            target: job.target,
            owner: job.owner,
            last_update: job.last_update,
            send_email: job.send_email
        })),
        None => Ok(None),
    }
}

pub fn update_job(job: JobEntity, job_id: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    let job = Job {
        id: job_id,
        name: job.name,
        code: job.code,
        description: job.description,
        schedule: job.schedule,
        target: job.target,
        owner: job.owner,
        last_update: job.last_update,
        send_email: job.send_email
    };

    diesel::update(jobs::table).set(&job).execute(conn)
}

pub fn delete_job(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    diesel::delete(jobs.filter(id.eq(uid))).execute(conn)
}
*/

