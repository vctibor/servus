use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;
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
    use crate::schema::users::dsl::*;
    //use crate::diesel::RunQueryDsl;

    let new_user = User {
        id: Uuid::new_v4(),
        name: user.name,
        email: user.email
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn get_users(conn: &PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;
    // use crate::diesel::RunQueryDsl;
    users.load::<User>(conn).expect("Error loading users")
}

pub fn get_user(uid: Uuid, conn: &PgConnection) -> Option<User> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid))
        .first::<User>(conn)
        .optional()
        .unwrap();

    user
}

pub fn update_user(user: User, conn: &PgConnection) {
    diesel::update(users::table).set(&user).execute(conn).unwrap();
}

pub fn delete_user(uid: Uuid, conn: &PgConnection) {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(uid))).execute(conn).unwrap();
}
