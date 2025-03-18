#[macro_use] extern crate rocket;
use std::path::Path;

use rocket::{fs::NamedFile, fs::relative, serde::json::Json};
pub mod juro;

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
    time: f64, 
    time_unit: TimeUnit, 
    contribution: Option<f64>) -> Json<Vec<juro::Result>> {

    let i: juro::Interest;
    if interest_unit == InterestUnit::Month {
        i = juro::Interest::from_month(interest)
    }
    else if interest_unit == InterestUnit::Year {
        i = juro::Interest::from_month(interest)
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![calc, main_page, static_file])
}

