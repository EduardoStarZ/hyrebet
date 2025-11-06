// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Int4,
        name -> Text,
        owner -> Varchar,
    }
}
