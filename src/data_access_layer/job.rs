use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::target::Target;
use super::user::User;
use super::tx_log::LogEntry;

/// Represents periodically scheduled task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub job_id: Uuid,
    pub name: String,

    /// Allows to create text to describe this task.
    pub description: String,

    /// Shell source code for task to be executed.
    pub code: String,
    pub schedule: String,
    pub target: Target,
    pub owner: User,
    pub log: Vec<LogEntry>
}