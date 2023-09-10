
// #![feature(proc_macro_hygiene, decl_macro)]
// extern crate diesel;
// mod schema;
// mod database;
// use crate::database::establish_connection;
// use diesel::prelude::*;
//
//
// #[derive(Debug, Queryable)]
// pub struct Human {
//     pub id: i32,
//     pub first_name: String,
//     pub last_name: String,
//     pub age: i32,
//     pub username: Option<String>, // Use Option for nullable columns
//     pub email: Option<String>,    // Use Option for nullable columns
//     pub location: Option<String>, // Use Option for nullable columns
// }
//
//
// fn main() {
//     let mut connection = establish_connection();
//     let humans = human::table
//         .load::<Human>(&mut connection)
//         .expect("Error loading humans");
//
//     println!("{:?}", humans);
// }
