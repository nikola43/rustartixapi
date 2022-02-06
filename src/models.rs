use diesel::prelude::*;
use diesel::Queryable;
use diesel::Insertable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub wallet_address: String,
}

/*
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: i32,
    pub wallet_address: &'a str,
}
*/