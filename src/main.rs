use rocket::http::Status;
use rocket::serde::Deserialize;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world! This is a Rocket server!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Sawasdee, {}!", name)
}

#[get("/weather/<city>")]
async fn weather(city: &str) -> Result<String, Status> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, "9912f3d53592c40f85e6302d68abf4be"
    );

    let response = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(_) => return Err(Status::InternalServerError),
    };

    let weather_data = match response.json::<WeatherResponse>().await {
        Ok(data) => data,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(format!(
        "The temperature in {} is {} degrees Celsius.",
        city, weather_data.main.temp
    ))
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", routes![index, hello, weather])
        .launch()
        .await
    {
        println!("Server failed to start: {}", e);
    }
}
