use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::pg::PgConnection;

use super::super::schema::*;


#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: Option<String>
}

/*
pub fn add_user(user: User) {

}
*/

pub fn get_users(conn: &PgConnection) -> Vec<User> {
    use super::super::schema::users::dsl::*;
    use crate::diesel::RunQueryDsl;
    users.load::<User>(conn).expect("Error loading users")
}

/*
pub fn get_user(id: Uuid) -> User {

}

pub fn delete_user(id: Uuid) {

}
*/