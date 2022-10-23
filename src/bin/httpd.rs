// /src/bin/httpd.rs

#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

use ghlcode_api_lib::data::query::interp_by_id;
use ghlcode_api_lib::data::query::Interpretations;
use ghlcode_api_lib::routes::api_testdata;
use ghlcode_api_lib::routes::health_check;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Interpretations::init())
        .mount("/", routes![health_check, api_testdata, interp_by_id])
}
