// @generated automatically by Diesel CLI.

diesel::table! {
    users (name) {
        name -> Varchar,
        join_date -> Timestamptz,
    }
}
