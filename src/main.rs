#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_codegen;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
  id: u32,
  name: String,
  email: String,
}

#[get("/")]
fn index() -> &'static str {
  "Welcome to our REST API!"
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
  let users: Vec<User> = vec![
    User { id: 1, name: "John Done".to_string(), email: "johndoe@mail.com".to_string() },
    User { id: 1, name: "John Done".to_string(), email: "johndoe@mail.com".to_string() },
  ];
  
  Json(users)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, get_users])
    .launch();
}