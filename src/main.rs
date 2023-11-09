mod users;
mod weather;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! This is a Rocket server!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Sawasdee, {}!", name)
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount(
            "/",
            routes![index, hello, weather::weather, users::get, users::create,],
        )
        .launch()
        .await
    {
        println!("Server failed to start: {}", e);
    }
}
