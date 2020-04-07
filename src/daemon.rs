use model::user;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&database_url).unwrap();

    let users = user::get_users(&conn);

    let users = serde_json::to_string(&users).unwrap();

    println!("{}", users);
}