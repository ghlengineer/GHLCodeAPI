// /src/lib/domain/query.rs

use rocket::futures::TryFutureExt;
use rocket::get;
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, Database};
use serde::{Deserialize, Serialize};

#[derive(Database)]
#[database("interpretations_db")]
pub struct Interpretations(sqlx::SqlitePool);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Interpretation {
    id: i64,
    date: String,
    code_edition: String,
    subject: String,
    keywords: String,
    reference: String,
    question: String,
    interpretation: String,
}

#[get("/codeapi/interpretations")]
pub async fn all_interps(mut db: Connection<Interpretations>) -> Json<Vec<Interpretation>> {
    let records = sqlx::query_as!(Interpretation, "SELECT * FROM interpretations")
        .fetch_all(&mut **db)
        .await
        .ok();
    Json(records.unwrap())
}

#[get("/codeapi/interpretations/<id>")]
pub async fn interp_by_id(
    mut db: Connection<Interpretations>,
    id: i64,
) -> Option<Json<Interpretation>> {
    sqlx::query!("SELECT * FROM interpretations WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| {
            Json(Interpretation {
                id: r.id,
                date: r.date,
                code_edition: r.code_edition,
                subject: r.subject,
                keywords: r.keywords,
                reference: r.reference,
                question: r.question,
                interpretation: r.interpretation,
            })
        })
        .await
        .ok()
}

#[get("/codeapi/interpretations/search/<keyword>")]
pub async fn interp_by_keyword(
    mut db: Connection<Interpretations>,
    keyword: &str,
) -> Option<Json<Interpretation>> {
    sqlx::query!(
        "SELECT * FROM interpretations WHERE keywords LIKE ?",
        keyword
    )
    .fetch_one(&mut **db)
    .map_ok(|r| {
        Json(Interpretation {
            id: r.id,
            date: r.date,
            code_edition: r.code_edition,
            subject: r.subject,
            keywords: r.keywords,
            reference: r.reference,
            question: r.question,
            interpretation: r.interpretation,
        })
    })
    .await
    .ok()
}
