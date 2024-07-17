use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;
use cess_rust_server::routes::configure;

#[tokio::main()]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenv().ok();

    // initialize SDK
    const MNEMONIC: &str = "pilot direct language fix marriage violin remember regular purity intact actress involve";

    let cess_self_node: bool = env::var("CESS_SELF")
        .map(|value| value == "true")  
        .unwrap_or(false);
    let cess_node_url: String = env::var("CESS_NODE_URL")
        .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let deoss_url: String = env::var("DEOSS_URL")
        .unwrap_or_else(|_| "default_deoss_url".to_string());
    let deoss_account: String = env::var("DEOSS_ACCOUNT")
        .unwrap_or_else(|_| "default_deoss_account".to_string());

    println!(" ================ ENV VARIABLES FOR CESS ============== \n");
    println!("\t CESS_SELF: {}", cess_self_node);
    println!("\t CESS_NODE_URL: {}", cess_node_url);
    println!("\t DEOSS_URL: {}", deoss_url);
    println!("\t DEOSS_ACCOUNT: {}", deoss_account);
    println!("\n");

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