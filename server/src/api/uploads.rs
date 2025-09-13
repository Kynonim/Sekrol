use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use futures_util::StreamExt;
use sqlx::{query_as, SqlitePool};
use uuid::Uuid;

use crate::models::struktur::VideoDatabase;

#[post("/api/videos")]
pub async fn upload_videos(koneksi: web::Data<SqlitePool>, mut payload: Multipart) -> impl Responder {
  let mut save_meta: Option<VideoDatabase> = None;

  while let Some(item) = payload.next().await {
    let mut field = item.unwrap();
    let content_dis = field.content_disposition();
    let name = content_dis.and_then(|c| c.get_filename().map(|s| s.to_string())).unwrap_or_else(|| "unknown".to_string());
    let content_type = field.content_type().map(|m| m.to_string()).unwrap_or_default();
    let id = Uuid::new_v4().to_string();
    let ext = std::path::Path::new(&name).extension().and_then(|s| s.to_str()).unwrap_or("");
    let stored_filename = if ext.is_empty() { format!("{}.bin", id) } else { format!("{}.{}", id, ext) };
    let filepath = format!("{}{}", std::env::var("SERVER_UPLOADS_URL").expect("upload file null"), stored_filename);

    let mut file = std::fs::File::create(&filepath).unwrap();
    let mut total_bytes: i64 = 0;
    
    while let Some(chunk) = field.next().await {
      let data = chunk.unwrap();
      file.write_all(&data).unwrap();
      total_bytes += data.len() as i64;
    }

    let created_at = Utc::now();
    sqlx::query("INSERT INTO videos (id, username, filename, description, content_type, size, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
      .bind(&id)
      .bind(&name)
      .bind(&stored_filename)
      .bind("Created by Riky Ripaldo")
      .bind(&content_type)
      .bind(total_bytes)
      .bind(created_at.to_rfc3339())
      .execute(koneksi.get_ref())
      .await.unwrap();

    save_meta = Some(VideoDatabase {
      id,
      username: name,
      filename: stored_filename,
      description: "Created by Riky Ripaldo".to_string(),
      content_type,
      size: total_bytes,
      created_at: created_at.to_rfc3339(),
    });
    break;
  }

  if let Some(meta) = save_meta {
    HttpResponse::Ok().json(meta)
  } else {
    HttpResponse::BadRequest().body("file kosong")
  }
}

#[get("/api/videos")]
pub async fn list_videos(koneksi: web::Data<SqlitePool>) -> impl Responder {
  let rows = query_as::<_, VideoDatabase>("SELECT * FROM videos ORDER BY created_at DESC")
    .fetch_all(koneksi.get_ref())
    .await.expect("database get error");

  let videos: Vec<VideoDatabase> = rows.into_iter().map(|data| VideoDatabase {
    id: data.id,
    username: data.username,
    filename: data.filename,
    description: data.description,
    content_type: data.content_type,
    size: data.size,
    created_at: chrono::DateTime::parse_from_rfc3339(&data.created_at).unwrap().with_timezone(&Utc).to_string()
  }).collect();

  HttpResponse::Ok().json(videos)
}