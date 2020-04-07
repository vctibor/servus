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
    pub last_update: NaiveDateTime,
    pub send_email: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxLog {
    pub id: Option<Uuid>,
    pub success: bool,
    pub time: NaiveDateTime,
    pub message: String
}

use std::error::Error;

pub type AnyError = Box<dyn Error + Send + Sync>;

use std::fmt;

#[derive(Debug)]
pub struct ServusError {
    details: String
}

impl ServusError {
    pub fn new(msg: &str) -> ServusError {
        ServusError{details: msg.to_string()}
    }
}

impl fmt::Display for ServusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ServusError {
    fn description(&self) -> &str {
        &self.details
    }
}

