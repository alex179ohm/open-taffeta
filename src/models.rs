// this file maps the table as a Rust structure
// decorators mean:
// (De)Serialize => can be used to (de)serialize data in JSON
// Queryable => can be used to represent entities in result set generated by queries
// ... there are many more ...

extern crate chrono;
use schema::users;
use chrono::{Duration, Utc};
use auth::Auth;
use config;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug, Default, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub active: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Default)]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub ring: bool,
    pub ring_ts: Option<i32>,
}

type Url = String;

#[derive(Serialize, Debug)]
pub struct UserAuth<'a> {
    id: i32,
    username: &'a str,
    email: &'a str,
    // bio: Option<&'a str>,
    // image: Option<&'a str>,
    token: String,
}

// #[derive(Serialize)]
// pub struct Profile {
//     username: String,
//     bio: Option<String>,
//     image: Option<String>,
//     following: bool,
// }

impl User {
    pub fn to_user_auth(&self) -> UserAuth {
        // FIXME: let exp = get_token_duration!();
        let exp = Utc::now() + Duration::days(60);
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }.token();

        UserAuth {
            id: self.id,
            username: &self.username,
            email: &self.email,
            // bio: self.bio.as_ref().map(String::as_str),
            // image: self.image.as_ref().map(String::as_str),
            token,
        }
    }

    // pub fn to_profile(self, following: bool) -> Profile {
    //     Profile {
    //         username: self.username,
    //         bio: self.bio,
    //         image: self.image,
    //         following,
    //     }
    // }
}
