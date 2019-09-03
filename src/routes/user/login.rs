use crate::db::Conn;
use crate::services::auth::auth_trait::AuthTrait;
use crate::services::auth::jwt::Claims;

use rocket::request::Form;

#[derive(Debug, FromForm, Clone)]
pub struct InputLogin {
    user: String,
    password: String,
}


#[post("/submit", data = "<input_login>")]
pub fn submit(conn: Conn, input_login: Form<InputLogin>) -> String {

    let claims: Claims = AuthTrait::new(1);


    conn.execute("INSERT INTO users (\"user\", \"password\") VALUES ($1, $2)",
                 &[&input_login.user, &input_login.password]).unwrap();

    format!("2")
}