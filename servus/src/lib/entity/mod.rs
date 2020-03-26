use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User<'a, 'b> {
    pub id: Option<Uuid>,
    pub name: &'a str,
    pub email: Option<&'b str>
}