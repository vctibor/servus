use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: Option<String>
}

pub fn add_user(user: NewUser, conn: &PgConnection)
                -> Result<User, diesel::result::Error>
{
    let new_user = User {
        id: Uuid::new_v4(),
        name: user.name,
        email: user.email
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn get_users(conn: &PgConnection)
                 -> Result<Vec<User>, diesel::result::Error>
{
    users.load::<User>(conn)
}

pub fn get_user(uid: Uuid, conn: &PgConnection)
                -> Result<Option<User>, diesel::result::Error>
{
    users
        .filter(id.eq(uid))
        .first::<User>(conn)
        .optional()
}

pub fn update_user(user: User, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    diesel::update(users::table).set(&user).execute(conn)
}

pub fn delete_user(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    diesel::delete(users.filter(id.eq(uid))).execute(conn)
}