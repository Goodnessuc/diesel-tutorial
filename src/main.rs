#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;

mod schema;
mod database;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::database::establish_connection;




#[derive(Debug, Queryable, Insertable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub item_type_id: i32,
    pub acquired_time: NaiveDateTime,
}




#[derive(Debug, Queryable, Insertable)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}

fn main() {



}