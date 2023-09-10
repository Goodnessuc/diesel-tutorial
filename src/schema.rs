// @generated automatically by Diesel CLI.

diesel::table! {
    human (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        age -> Integer,
        username -> Nullable<Text>,
        email -> Nullable<Text>,
        location -> Nullable<Text>,
    }
}
