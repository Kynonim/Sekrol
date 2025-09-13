use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct VideoDatabase {
  pub id: String,
  pub username: String,
  pub filename: String,
  pub description: String,
  pub content_type: String,
  pub size: i64,
  pub created_at: String,
}