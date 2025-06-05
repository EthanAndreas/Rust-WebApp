use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_username(
        pool: &sqlx::PgPool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                password_hash,
                COALESCE(created_at, CURRENT_TIMESTAMP) as "created_at!: DateTime<Utc>"
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let password_hash = bcrypt::hash(password, 10).unwrap();
        let now = Utc::now();

        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                username,
                email,
                password_hash,
                created_at as "created_at!: DateTime<Utc>"
            "#,
            id,
            username,
            email,
            password_hash,
            now
        )
        .fetch_one(pool)
        .await
    }
}