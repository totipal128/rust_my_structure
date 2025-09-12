// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        address -> Varchar,
        email -> Varchar,
        is_agent -> Bool,
        work_unit_id -> Nullable<Int4>,
    }
}
