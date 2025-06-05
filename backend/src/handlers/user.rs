use actix_web::{get, HttpResponse, Result};

#[get("/profile")]
pub async fn profile() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Profile page"))
}