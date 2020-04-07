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