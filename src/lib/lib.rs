#[macro_use] extern crate diesel;

use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod schema;
pub mod entity;
pub mod persistence;
pub mod execution;