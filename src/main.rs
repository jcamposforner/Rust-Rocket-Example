#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use db::Conn;
use dotenv::dotenv;
use rocket::request::Form;
use std::env;

mod db;

#[derive(Debug, FromForm, Clone)]
struct Message {
    message: String,
    role: Option<u8>,
}

#[post("/submit", data = "<message>")]
fn submit(conn: Conn, message: Form<Message>) -> String {
    let t = message.clone();
    let b = message.role.unwrap_or(0);
    println!("{}", b);
    format!("{}", b)
}

fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);
    rocket::ignite().manage(pool).mount("/", routes![submit])
}

fn main() {
    rocket().launch();
}
