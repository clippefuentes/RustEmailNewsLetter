use RustEmailNewsLetter::startup::run;
use RustEmailNewsLetter::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::{Connection, PgPool};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port); 
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}