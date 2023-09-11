// #![feature(proc_macro_hygiene, decl_macro)]
// extern crate diesel;
//
// mod schema;
// mod database;
//
// use diesel::prelude::*;
// use crate::database::establish_connection;
//
// #[derive(Queryable, Insertable)]
// #[table_name = "schema::human"]
// pub struct NewHuman {
//     pub first_name: String,
//     pub last_name: String,
//     pub age: i32,
//     pub username: String,
//     pub email: String,
//     pub location: String,
// }
//
// fn main() {
//     let mut connection = establish_connection();
//
//     // Create a vector of NewHuman records (up to 10 rows), some of which may be duplicates
//     let new_humans = vec![
//         NewHuman {
//             first_name: String::from("John"),
//             last_name: String::from("Doe"),
//             age: 30,
//             username: String::from("johndoe"),
//             email: String::from("john.doe@example.com"),
//             location: String::from("New York"),
//         },
//         NewHuman {
//             first_name: String::from("Alice"),
//             last_name: String::from("Smith"),
//             age: 28,
//             username: String::from("alicesmith"),
//             email: String::from("alice.smith@example.com"),
//             location: String::from("Los Angeles"),
//         },
//         NewHuman {
//             first_name: String::from("David"),
//             last_name: String::from("Johnson"),
//             age: 35,
//             username: String::from("davidjohnson"),
//             email: String::from("david.johnson@example.com"),
//             location: String::from("Chicago"),
//         },
//     ];
//
//     // Insert the new_humans vector into the database
//     for new_human in new_humans {
//         diesel::insert_into(schema::human::table)
//             .values(&new_human)
//             .execute(&mut connection)
//             .expect("Error inserting human");
//     }
// }
