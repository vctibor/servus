use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Represents login to remote machine.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub target_id: Uuid,
    pub name: String,
    pub username: String,
    pub url: String,
    pub port: i32
}

/*
pub fn add_target(m: Target) {

}

pub fn get_targets() -> Vec<Target> {

}

pub fn get_target(id: Uuid) -> Target {

}

pub fn delete_target(id: Uuid) {

}
*/