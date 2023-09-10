// #![feature(proc_macro_hygiene, decl_macro)]
//
//
//
//
//
// mod schema;
// mod database;
//
// use crate::database::establish_connection;
// use diesel::Insertable;
// use diesel::{Queryable, RunQueryDsl};
//
// // Specify the fully-qualified path for the table_name attribute
// #[derive(Queryable, Insertable)]
// #[diesel(table_name = schema::human)]
// pub struct NewHuman {
//     pub first_name: String,
//     pub last_name: String,
//     pub age: i32,
//     pub username: String,
//     pub email: String,
//     pub location: String,
// }
//
//
//
//
//
//
//
// fn main() {
//     let mut connection = establish_connection();
//     let new_human = NewHuman {
//         first_name: String::from("John"),
//         last_name: String::from("Doe"),
//         age: 30,
//         username: String::from("johndoe"),
//         email: String::from("john.doe@example.com"),
//         location: String::from("New York"),
//     };
//
//     diesel::insert_into(schema::human::table).values(&new_human).execute(&mut connection).expect("Error saving new human");
// }
