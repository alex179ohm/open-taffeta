// this file maps the DB tables as Rust structures
// decorators mean:
// (De)Serialize => can be used to (de)serialize data in JSON
// Queryable => can be used to represent entities in result set generated by queries
// ... there are many more ...

use serde_derive::{Serialize, Deserialize};
use crate::schema::users;
use crate::auth::Auth;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug, Default, Insertable, AsChangeset, Identifiable, PartialEq)]
pub struct User {
    pub id: i32,
    pub password: String,
    pub email: String,
    pub active: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserAuth {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub active: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Default)]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub buzzer_url: String,
    pub ring: bool,
    pub ring_ts: Option<i32>,
}

impl User {
    pub fn to_user_auth(&self) -> UserAuth {
        let exp = get_token_duration!();
        let token = Auth {
            id: self.id,
            email: self.email.clone(),
            exp: exp.timestamp(),
        }.token();

        UserAuth {
            id: self.id,
            email: self.email.clone(),
            token: token,
            active: self.active
        }
    }
}
