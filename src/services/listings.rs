use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

use crate::utils::helpers::DbPool;

#[post("/listings")]
async fn get_listings(pool: web::Data<DbPool>, session: Session) -> Resuls<impl Responder> {
    Ok(HttpResponse::Ok().json(json!({"status": "OK", "message": "Listings!"})))
}
