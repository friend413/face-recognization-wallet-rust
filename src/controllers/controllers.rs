use actix_web::{web, HttpResponse, Responder};
use cess_rust_sdk::core::utils::account::get_pair_address_as_ss58_address;
use serde::{Deserialize, Serialize};
use sp_keyring::sr25519::sr25519::Pair;
use diesel::prelude::*;

use crate::{
    controllers::accounts::{generate_mnemonic, get_pair},
    databases::*,
    databases::models::Account,
    schema::account::dsl::*,
    jwt::generate_token
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetWalletInfo {
    uid: i64,
    address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateWalletInfo {
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

pub async fn get_wallet_post(info: web::Json<GetWalletInfo>) -> impl Responder {
    let connection = &mut establish_connection();

    println!("heere came");
    let results = account
        .filter(uid.eq(info.clone().uid))
        .filter(address.eq(info.clone().address))
        .limit(1)
        .select(Account::as_select())
        .load(connection)
        .expect("Error loading account");
    println!("heere came 2");
    if results.len() == 0 {
        let response_message = WalletResponse{
            result: "Error".to_string(),
            msg: "Can not find the account".to_string(),
            wallet_address: "".to_string(),
            token: "".to_string()
        };
        return HttpResponse::Ok().content_type("application/json").json(response_message);
    }
    println!("heere came 3");
    match generate_token(info.address.clone(), info.uid.clone()) {
        Ok(jtoken) => {
            let response_message = WalletResponse {
                result: "Success".to_string(),
                msg: "Got wallet successfully".to_string(),
                wallet_address: info.address.clone(),
                token: jtoken
            };
            println!("heere came 4");
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
            println!("heere came5");
            HttpResponse::Ok().json(response_message)
        }
    }
}

pub async fn create_wallet_post(info: web::Json<CreateWalletInfo>) -> impl Responder {
    let mnem: Option<String>;
    println!("======================  create wallet 1 ");
    match generate_mnemonic() {
        Ok(t) => mnem = Some(t),
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
    println!("======================  create wallet 2 ");
    let pair: Pair;
    match get_pair(&mnem.clone().unwrap(), None) {
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
    println!("======================  create wallet 3 ");
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
    println!("======================  create wallet 4 ");
    match generate_token(address_to_fund.clone(), info.uid.clone()) {
        Ok(jtoken) => {
            let connection = &mut establish_connection();

            let myaccount = create_account(connection, info.uid.clone(), Some(&mnem.unwrap()), Some(&address_to_fund.clone()), Some(&jtoken.clone()));
            
            println!("test account: {:?}", myaccount.clone());
            let response_message = WalletResponse {
                result: "Success".to_string(),
                msg: "Created wallet successfully".to_string(),
                wallet_address: address_to_fund,
                token: jtoken
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

