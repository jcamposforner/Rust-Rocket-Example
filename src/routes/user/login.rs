use crate::db::Conn;
use rocket::request::Form;
use jsonwebtoken::{encode, Header};
use serde_derive::{Serialize, Deserialize};
use std::env;
use chrono::{Utc, DateTime};

#[derive(Debug, FromForm, Clone)]
pub struct InputLogin {
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
   sub: String,
   company: String,
   created_at: i64
}


#[post("/submit", data = "<input_login>")]
pub fn submit(conn: Conn, input_login: Form<InputLogin>) -> String {
    let secret_key = env::var("SECRET_KEY").expect("NO SECRET KEY AVAILABLE");

    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        created_at: DateTime::timestamp(&Utc::now())
    };

    let token = encode(&Header::default(), &my_claims, secret_key.as_ref()).unwrap();
    format!("{}", token)
}