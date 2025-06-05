use actix_web::{post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use bcrypt::verify;

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    username: String,
}

#[post("/register")]
pub async fn register(
    data: web::Json<RegisterData>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse> {
    match User::create(&pool, &data.username, &data.email, &data.password).await {
        Ok(user) => Ok(HttpResponse::Ok().json(AuthResponse {
            token: "session-token".to_string(),
            username: user.username,
        })),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[post("/login")]
pub async fn login(
    data: web::Json<LoginData>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse> {
    let user = match User::find_by_username(&pool, &data.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    match verify(&data.password, &user.password_hash) {
        Ok(true) => Ok(HttpResponse::Ok().json(AuthResponse {
            token: "session-token".to_string(),
            username: user.username,
        })),
        Ok(false) => Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}