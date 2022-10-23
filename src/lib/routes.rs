// /src/lib/routes.rs

use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::data::testdata::TestData;

#[get("/health_check")]
pub fn health_check() -> (Status, &'static str) {
    (Status::Ok, "200 Ok")
}

#[get("/test_data")]
pub fn api_testdata() -> Json<Vec<TestData>> {
    TestData::get_data()
}
