//
// #![feature(proc_macro_hygiene, decl_macro)]
//
// extern crate diesel;
//
// mod schema;
// mod database;
//
// use diesel::prelude::*;
//
//
// use crate::database::establish_connection;




// #[derive(Queryable, AsChangeset, Identifiable)]
// #[diesel(table_name = schema::human)]
// struct UpdateHuman {
//     pub id: i32,
//     pub first_name: String,
//     pub last_name: String,
//     pub age: i32,
//     pub username: Option<String>,
//     pub email: Option<String>,
//     pub location: Option<String>,
// }
//
// fn main() {
//     use crate::schema::human::dsl::*;
//     let mut connection = establish_connection();
//
//     let id_to_update = 1; // Replace with the ID of the human you want to update
//
//     let human_update = UpdateHuman {
//         id: 2,
//         first_name: "Janeen".to_string(),
//         last_name: "Elixir".to_string(),
//         age: 30,
//         username: Some("elixir".to_string()),
//         email: Some("jelixir@example.com".to_string()),
//         location: Some("Pars".to_string()),
//     };
//
//     let updated_rows = diesel::update(human.find(id_to_update))
//         .set(&human_update)
//         .execute(&mut connection)
//         .expect("Failed to update student");
//
//     println!("{:?}", updated_rows);
// }
