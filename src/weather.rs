use rocket::http::Status;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

#[get("/weather/<city>")]
pub async fn weather(city: &str) -> Result<String, Status> {
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
