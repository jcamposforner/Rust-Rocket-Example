#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use db::Conn;
use dotenv::dotenv;
use std::env;
use auth::Authorization;
use routes::user::login;

mod db;
mod auth;
mod routes;
mod services;

#[get("/sensitive")]
fn sensitive(_key: Authorization) -> &'static str {
    "Sensitive data."
}

fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);
    rocket::ignite().manage(pool).mount("/", routes![login::submit, sensitive])
}

fn main() {
    rocket().launch();
}
