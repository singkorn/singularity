use rocket::get;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    // other fields...
}

#[get("/<id>")]
pub fn get(id: i32) -> String {
    format!("Get user with id {}", id)
}

#[post("/", data = "<user>")]
pub fn create(user: Json<User>) -> String {
    format!("Create user with name {}", user.name)
}
