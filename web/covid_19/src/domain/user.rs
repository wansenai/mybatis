use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub sex: u8,
    pub brithday: String,
    pub status: u8,
}

impl User {
    pub fn new(id: String, username: String, password: String, name: String, sex: u8, brithday: String, status: u8) -> User {
        User {
            id,
            username,
            password,
            name,
            sex,
            brithday,
            status,
        }
    }
}