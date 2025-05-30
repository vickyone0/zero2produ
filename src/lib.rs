pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod email_client;
// use actix_web::{web, App, HttpResponse, HttpServer};
// use actix_web::dev::Server;
// use std::net::TcpListener;

// async fn health_check() -> HttpResponse {
// HttpResponse::Ok().finish()
// }

// #[derive(serde::Deserialize)]
// struct FormData {
//     name: String,
//     email: String,
// }

// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// pub  fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//       let server=  HttpServer::new(|| {
//         App::new()
//         .route("/health_check", web::get().to(health_check))
//         .route("/subscriptions", web::post().to(subscribe))
//         })
//         .listen(listener)?
//         .run();
//     Ok(server)
// }
