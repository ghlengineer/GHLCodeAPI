// /src/bin/httpd.rs

#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;

use ghlcode_api_lib::data::query::all_interps;
use ghlcode_api_lib::data::query::interp_by_id;
use ghlcode_api_lib::data::query::interp_by_keyword;
use ghlcode_api_lib::data::query::Interpretations;
use ghlcode_api_lib::routes::api_health_check;
use ghlcode_api_lib::routes::api_test_data;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Interpretations::init()).mount(
        "/",
        routes![
            api_health_check,
            api_test_data,
            all_interps,
            interp_by_id,
            interp_by_keyword
        ],
    )
}
