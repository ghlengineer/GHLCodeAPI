// src/lib/domain/data.rs

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TestData {
    id: u32,
    content: String,
}

impl TestData {
    pub fn get_data() -> Json<Vec<TestData>> {
        let testdata = vec![TestData {
            id: 1,
            content: "Some Test Data".to_string(),
        }];
        Json(testdata)
    }
}
