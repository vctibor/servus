use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entity::User as UserEntity;
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Identifiable, Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
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

pub fn add_user(user: UserEntity, conn: &PgConnection)
                -> Result<UserEntity, diesel::result::Error>
{
    let new_user = User {
        id: Uuid::new_v4(),
        name: user.name,
        email: user.email
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)?;

    Ok(UserEntity {
        id: Some(new_user.id),
        name: new_user.name,
        email: new_user.email
    })
}

pub fn get_users(conn: &PgConnection)
                 -> Result<Vec<UserEntity>, diesel::result::Error>
{
    let user_table: Vec<User> = users.load::<User>(conn)?;

    let mut entities = Vec::with_capacity(user_table.len());

    for user in user_table.iter() {
        entities.push(UserEntity {
            id: Some(user.id),
            name: user.name.to_owned(),
            email: user.email.to_owned()
        });
    }
    
    Ok(entities)
}

pub fn get_user(uid: Uuid, conn: &PgConnection)
                -> Result<Option<UserEntity>, diesel::result::Error>
{
    let user = users
        .filter(id.eq(uid))
        .first::<User>(conn)
        .optional()?;

    match user {
        Some(user) => Ok(Some(UserEntity {
            id: Some(user.id),
            name: user.name,
            email: user.email
        })),
        None => Ok(None),
    }
}

pub fn update_user(user: UserEntity, user_id: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    let user = User {
        id: user_id,
        name: user.name,
        email: user.email
    };

    diesel::update(users::table).set(&user).execute(conn)
}

pub fn delete_user(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    diesel::delete(users.filter(id.eq(uid))).execute(conn)
}