// @generated automatically by Diesel CLI.

diesel::table! {
    users (name) {
        name -> Varchar,
        password -> Text,
        join_date -> Timestamptz,
    }
}
