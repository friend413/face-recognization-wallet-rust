use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use models::{NewAccount, Account};
use crate::schema::account;

pub mod models;
// pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_account(conn: &mut PgConnection, uid: i64, mnemonic: Option<&str>, address: Option<&str>, token: Option<&str>) -> Account {

    let new_post = NewAccount { uid, mnemonic, address, token };

    diesel::insert_into(account::table)
        .values(&new_post)
        // .returning(account::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}