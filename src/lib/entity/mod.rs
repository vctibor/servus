pub mod error;
pub use error::*;

use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Machine {
    pub id: Option<Uuid>,
    pub name: String,
    pub username: String,
    pub url: String,
    pub port: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: Option<Uuid>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub schedule: String,
    pub target: Machine,
    pub owner: User,
    pub last_update: Option<NaiveDateTime>,
    pub send_email: bool,
    pub last_status: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxLog {
    pub id: Option<Uuid>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub success: bool,
    pub time: NaiveDateTime,
    pub message: String,
    pub job: Uuid,
    pub job_name: Option<String>
}

