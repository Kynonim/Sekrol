use actix_web::{get, HttpResponse, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
  HttpResponse::Ok().body("Respon berhasil")
}