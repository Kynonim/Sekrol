pub mod struktur;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn koneksi() -> SqlitePool {
  let dbpath = std::env::var("SERVER_DATABASE_URL").expect("database tidak ada");
  let dbname = std::env::var("SERVER_DATABASE_NAME").expect("nama db tidak ada");
  let uploadpath = std::env::var("SERVER_UPLOADS_URL").expect("upload folder kosong");

  if !std::path::Path::new(&dbpath).exists() {
    std::fs::create_dir_all(&dbpath).expect("gagal buat path db");
    std::fs::File::create(format!("{}{}", &dbpath, &dbname)).expect("gagal buat file db");
  }

  let sqlite = SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&format!("sqlite:{}{}", dbpath, dbname))
    .await.unwrap();


  if !std::path::Path::new(&uploadpath).exists() {
    std::fs::create_dir_all(uploadpath).expect("gagal membuat uploads folder");
  }

  sqlx::query("
    CREATE TABLE IF NOT EXISTS videos (
      id TEXT PRIMARY KEY,
      username TEXT NOT NULL,
      filename TEXT NOT NULL,
      description TEXT,
      content_type TEXT,
      size INTEGER,
      created_at TEXT NOT NULL
    );
  ").execute(&sqlite).await.expect("kesalahan sql");

  return sqlite;
}