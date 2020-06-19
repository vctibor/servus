#![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

#[macro_use] extern crate diesel;

pub mod schema;
pub mod entity;
pub mod persistence;
