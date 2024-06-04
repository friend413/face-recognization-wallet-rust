use actix_web::{web, HttpResponse, Responder};
use cess_rust_sdk::core::utils::account::get_pair_address_as_ss58_address;
use serde::{Deserialize, Serialize};
use sp_keyring::sr25519::sr25519::Pair;
use subxt_signer::bip39::Mnemonic;
use crate::{
    controllers::accounts::{generate_mnemonic, get_pair},
    jwt::{generate_token, is_valid}
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WalletInfo {
    uid: i64
}

#[derive(Serialize, Debug)]
pub struct WalletResponse {
    result: String,
    msg: String,
    wallet_address: String,
    token: String
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the face-recognization rust server!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: Running")
}

pub async fn get_wallet_post(info: web::Json<WalletInfo>) -> impl Responder {
    let response_message = format!(
        "address: {}",
        info.uid
    );
    HttpResponse::Ok().body(response_message)
}

pub async fn create_wallet_post(info: web::Json<WalletInfo>) -> impl Responder {
    let mnemonic: Option<String>;

    match generate_mnemonic() {
        Ok(t) => mnemonic = Some(t),
        Err(_) => {
            let response_message = WalletResponse{
                result: "Error".to_string(),
                msg: "Internal error on `generate_mnemonic`".to_string(),
                wallet_address: "".to_string(),
                token: "".to_string()
            };
            return HttpResponse::Ok().content_type("application/json").json(response_message);
        }
    };

    let pair: Pair;
    match get_pair(&mnemonic.clone().unwrap(), None) {
        Ok(t) => pair = t,
        Err(_) => {
            let response_message = WalletResponse{
                result: "Error".to_string(),
                msg: "Internal error on `get_pair`".to_string(),
                wallet_address: "".to_string(),
                token: "".to_string()
            };
            return HttpResponse::Ok().content_type("application/json").json(response_message);
        }
    };

    let address_to_fund: String;
    match get_pair_address_as_ss58_address(pair) {
        Ok(t) => address_to_fund = t,
        Err(_) => {
            let response_message = WalletResponse{
                result: "Error".to_string(),
                msg: "Internal error on `get_pair_address_as_ss58_address`".to_string(),
                wallet_address: "".to_string(),
                token: "".to_string()
            };
            return HttpResponse::Ok().content_type("application/json").json(response_message);
        }
    }

    match generate_token(address_to_fund.clone(), info.uid.clone()) {
        Ok(token) => {
            let response_message = WalletResponse {
                result: "Success".to_string(),
                msg: "Created wallet successfully".to_string(),
                wallet_address: address_to_fund,
                token
            };
        
            println!("test response_message: {:?}", response_message);
            HttpResponse::Ok().json(response_message)
        },
        Err(_) => {
            let response_message = WalletResponse{
                result: "Error".to_string(),
                msg: "Internal error on `generate_token`".to_string(),
                wallet_address: "".to_string(),
                token: "".to_string()
            };
        
            HttpResponse::Ok().json(response_message)
        }
    }
}