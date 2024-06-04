use diesel::prelude::*;
use crate::schema::account;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i64,
    pub uid: i64,
    pub mnemonic: Option<String>,
    pub address: Option<String>,
    pub token: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub uid: i64,
    pub mnemonic: Option<&'a str>,
    pub address: Option<&'a str>,
    pub token: Option<&'a str>
}