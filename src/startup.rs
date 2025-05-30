use crate::routes::{health_check, subscribe};
use crate::email_client::{self, EmailClient};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;


pub fn run(listener: TcpListener, db_pool: PgPool,email_client:EmailClient,) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
