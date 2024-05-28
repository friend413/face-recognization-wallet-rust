use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;
use cess_rust_server::routes::configure;

#[tokio::main()]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenv().ok();

    let explorer_server_port: String = env::var("EXPLORER_SERVER_PORT")
        .unwrap_or_else(|_| "8799".to_string());
    let explorer_server_host: String = env::var("EXPLORER_SERVER_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Hello, world!");
    let _ = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .configure(configure)
    })
    .bind(format!("{explorer_server_host}:{explorer_server_port}"))? // Bind server to localhost:8080
    .run()
    .await;

    Ok(())
}