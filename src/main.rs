#[macro_use] extern crate rocket;
use std::path::Path;

use rocket::{fs::NamedFile, fs::relative, serde::json::Json};
pub mod juro;
use reqwest::{Client, Response};
use serde::Deserialize;

#[derive(Debug, PartialEq, FromFormField)]
enum InterestUnit {Month, Year}

#[derive(Debug, PartialEq, FromFormField)]
enum TimeUnit {Month, Year}

#[get("/static/<f>")]
async fn static_file(f: &str) -> Option<NamedFile> {
    let path = Path::new(relative!("static")).join(f);
    NamedFile::open(path).await.ok()
}
#[get("/")]
async fn main_page() -> Option<NamedFile> {
    let path = Path::new(relative!("static")).join("index.html");
    NamedFile::open(path).await.ok()
}

#[get("/juro?<initial>&<interest>&<interest_unit>&<time>&<time_unit>&<contribution>")]
fn calc(initial: f64,
    interest: f64, 
    interest_unit: InterestUnit, 
    time: i32, 
    time_unit: TimeUnit, 
    contribution: Option<f64>) -> Json<Vec<juro::Result>> {

    let i: juro::Interest;
    if interest_unit == InterestUnit::Month {
        i = juro::Interest::from_month(interest)
    }
    else if interest_unit == InterestUnit::Year {
        i = juro::Interest::from_effective_year(interest)
    } else {
        panic!("Error")
    }

    let t: juro::Time;
    if time_unit == TimeUnit::Month {
        t = juro::Time::from_month(time);
    }
    else if time_unit == TimeUnit::Year {
        t = juro::Time::from_year(time);
    } else {
        panic!("Error")
    }

    let calc: juro::Calculation = juro::Calculation{
        initial,
        interest: i,
        contribution_per_month: contribution.unwrap_or(0.0)};

    let result: Vec<juro::Result> = calc.by_month(t);

    return Json(result);
}

#[derive(Deserialize, serde::Serialize, Debug)]
struct ApiResponse {
    data: String,
    valor: String,
}

#[get("/selic")]
async fn get_selic() -> String {
    let client = Client::new();
    let res = client.get("google.com")
        .send()
        .await
        .unwrap();

    let selic: Vec<ApiResponse> = res.json()
        .await
        .unwrap_or(vec![ApiResponse{data: "01/01/01".to_string(), valor:"10.0".to_string()}]);

    return selic[0].valor.clone();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![calc, main_page, static_file, get_selic])
}

