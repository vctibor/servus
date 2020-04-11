use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::entity::TxLog as LogEntry;
use crate::entity::AnyError;
use crate::schema::tx_log;
use crate::schema::tx_log::dsl::*;
use crate::persistence::get_job;
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name="tx_log"]
pub struct Log {
    pub id: Uuid,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub success: bool,
    pub time: NaiveDateTime,
    pub message: String,
    pub job: Uuid
}


pub fn write_log(entry: LogEntry, conn: &PgConnection) 
    -> Result<LogEntry, AnyError>
{
    let new_entry = Log {
        id: Uuid::new_v4(),
        stdout: entry.stdout,
        stderr: entry.stderr,
        success: entry.success,
        time: entry.time,
        message: entry.message,
        job: entry.job
    };

    diesel::insert_into(tx_log)
        .values(&new_entry)
        .execute(conn)?;

    let entry = LogEntry {
        id: Some(new_entry.id),
        stdout: new_entry.stdout,
        stderr: new_entry.stderr,
        success: new_entry.success,
        time: new_entry.time,
        message: new_entry.message,
        job: new_entry.job,
        job_name: None
    };

    Ok(entry)
}

/// Get log entries specific for given job_id,
/// allows paging with offset and size.
/// Entris are ordered by time from newest to oldest.
pub fn get_job_log(job_id: Uuid, offset: i64, size: i64, conn: &PgConnection)
    -> Result<Vec<LogEntry>, AnyError>
{
    let results = tx_log.filter(job.eq(job_id))
                        .order(time.desc())
                        .offset(offset)
                        .limit(size)
                        .load::<Log>(conn)?;
    
    let mut res_vec= Vec::with_capacity(results.len());

    for result in results {
        res_vec.push(LogEntry {
            id: Some(result.id),
            stdout: result.stdout,
            stderr: result.stderr,
            success: result.success,
            time: result.time,
            message: result.message,
            job: result.job,
            job_name: None
        })
    }

    let res_vec = res_vec;
    Ok(res_vec)
}

/// Get all log entries,
/// allows paging with offset and size.
/// Entris are ordered by time from newest to oldest.
pub fn get_log(offset: i64, size: i64, conn: &PgConnection)
    -> Result<Vec<LogEntry>, AnyError>
{
    let results = tx_log.order(time.desc())
                        .offset(offset)
                        .limit(size)
                        .load::<Log>(conn)?;

    let mut res_vec= Vec::with_capacity(results.len());
    for result in results {
        let job_entity = get_job(result.job, &conn);
        
        let job_name = match job_entity {
            Ok(j) => match j {
                Some(jj) => Some(jj.name),
                None => None,
            },
            Err(_) => None
        };

        res_vec.push(LogEntry {
            id: Some(result.id),
            stdout: result.stdout,
            stderr: result.stderr,
            success: result.success,
            time: result.time,
            message: result.message,
            job: result.job,
            job_name: job_name
        })
    }

    let res_vec = res_vec;
    Ok(res_vec)
}