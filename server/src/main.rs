mod api;
mod models;

use actix_web::{web, App, HttpServer};
use crate::{api::uploads::{list_videos, upload_videos}, models::koneksi};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	let host = std::env::var("SERVER_HOST").expect("SERVER_HOST tidak ada di environment");
	let port = std::env::var("SERVER_PORT").expect("SERVER_PORT tidak ada di environment");
	let port_parse: Result<u16, _> = port.parse();

	let koneksi = koneksi().await;

  HttpServer::new(move || {
		App::new()
		  .app_data(web::Data::new(koneksi.clone()))
		  .service(upload_videos)
			.service(list_videos)
			.service(actix_files::Files::new("/videos/", "./res/videos").show_files_listing())
			.service(actix_files::Files::new("/", "../client/dist").index_file("index.html"))
			.default_service(web::to(|| async {
				actix_files::NamedFile::open("../client/dist/index.html").map(|f| f.use_last_modified(false))
				  .map_err(|_| actix_web::error::ErrorNotFound("tidak ditemukan"))
			}))
	})
	.bind((host, port_parse.expect("Parse gagal karena bukan integer")))?
	.run().await
}
