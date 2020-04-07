use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub tx_log_id: Uuid,
    pub success: bool,
    pub time: i32, // TODO: use chrono crate
    pub message: String
}