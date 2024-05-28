use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WalletInfo {
    feature_info: String
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the face-recognization rust server!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: Running")
}

pub async fn get_wallet_post(info: web::Json<WalletInfo>) -> impl Responder {
    let response_message = format!(
        "Got info for: feature - {}",
        info.feature_info
    );
    HttpResponse::Ok().body(response_message)
}

pub async fn create_wallet_post(info: web::Json<WalletInfo>) -> impl Responder {
    let response_message = format!(
        "Created wallet for: feature_info - {}",
        info.feature_info
    );
    HttpResponse::Ok().body(response_message)
}