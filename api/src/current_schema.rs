// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id, owner) {
        id -> Int4,
        reply -> Nullable<Int4>,
        repost -> Nullable<Int4>,
        owner -> Varchar,
        contents -> Text,
        likes -> Int4,
        time -> Timestamptz,
    }
}
