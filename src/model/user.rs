use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;
use diesel::pg::PgConnection;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
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
    use crate::diesel::RunQueryDsl;

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
    use crate::diesel::RunQueryDsl;
    users.load::<User>(conn).expect("Error loading users")
}

/*
pub fn get_user(id: Uuid, conn: &PgConnection) -> User {

}

pub fn update_user(user: User, conn: &PgConnection) -> User {

}

pub fn delete_user(id: Uuid, conn: &PgConnection) {

}
*/