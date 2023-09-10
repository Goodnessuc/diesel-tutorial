#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;


mod schema;
mod database;

#[derive(Debug, Queryable)]
pub struct Human {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub username: String,
    pub email: String,
    pub location: String,
}


fn main(){

}