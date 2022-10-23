// /src/lib/routes.rs

use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::data::testdata::TestData;

#[get("/api/health_check")]
pub fn api_health_check() -> (Status, &'static str) {
    (Status::Ok, "200 Ok")
}

#[get("/api/test_data")]
pub fn api_test_data() -> Json<Vec<TestData>> {
    TestData::get_data()
}
