// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        address -> Varchar,
        is_agent -> Bool,
    }
}
