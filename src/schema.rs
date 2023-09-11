// @generated automatically by Diesel CLI.

diesel::table! {
    item (id) {
        id -> Nullable<Integer>,
        name -> Text,
        item_type_id -> Integer,
        acquired_time -> Timestamp,
    }
}

diesel::table! {
    item_type (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::joinable!(item -> item_type (item_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    item,
    item_type,
);
