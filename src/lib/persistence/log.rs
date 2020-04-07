use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::entity::TxLog as LogEntry;
use crate::entity::AnyError;
use crate::schema::tx_log;
use crate::schema::tx_log::dsl::*;
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[table_name="tx_log"]
pub struct Log {
    pub id: Uuid,
    pub success: bool,
    pub time: NaiveDateTime,
    pub message: String,
    pub job: Uuid
}

/*
pub fn write_log(entry: LogEntry, conn: &PgConnection) {

}
*/

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
            success: result.success,
            time: result.time,
            message: result.message,
            job: result.job
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
        res_vec.push(LogEntry {
            id: Some(result.id),
            success: result.success,
            time: result.time,
            message: result.message,
            job: result.job
        })
    }

    let res_vec = res_vec;
    Ok(res_vec)
}