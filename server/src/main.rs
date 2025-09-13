mod api;

use actix_web::{App, HttpServer};
use crate::api::get_users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	let host = std::env::var("SERVER_HOST").expect("SERVER_HOST tidak ada di environment");
	let port = std::env::var("SERVER_PORT").expect("SERVER_PORT tidak ada di environment");
	let port_parse: Result<u16, _> = port.parse();

  HttpServer::new(|| {
		App::new()
		  .service(get_users)
			.service(actix_files::Files::new("/", "../client/dist").index_file("index.html"))
	})
	.bind((host, port_parse.expect("Parse gagal karena bukan integer")))?
	.run().await
}
