// @generated automatically by Diesel CLI.

diesel::table! {
    likes (route, user) {
        route -> Varchar,
        user -> Varchar,
    }
}

diesel::table! {
    posts (id, owner) {
        id -> Int4,
        reply -> Nullable<Varchar>,
        repost -> Nullable<Varchar>,
        owner -> Varchar,
        contents -> Text,
        total_likes -> Int4,
        time -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(likes, posts,);
