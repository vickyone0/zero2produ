use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};
use zero2prod::email_client::EmailClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());


        let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address.");
        let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email
        );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    run(listener, connection_pool,email_client)?.await
}
