#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;
mod database;

use diesel::prelude::*;

use crate::schema::human::dsl::human;

use crate::database::establish_connection;

fn main() {
    let id_to_delete = 1;


    let mut connection = establish_connection();

    let deleted_rows = diesel::delete(human.find(id_to_delete)).execute(&mut connection)?;


    println!("Deleted {} rows.", deleted_rows);
}

