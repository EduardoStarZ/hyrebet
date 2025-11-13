diesel::table!{
    posts(id, owner) {
        id -> Integer,
        reply -> Nullable<VarChar>,
        repost -> Nullable<VarChar>,
        owner -> VarChar,
        contents -> Text,
        total_likes -> Integer,
        time -> Timestamptz
    }
}

diesel::table! {
    likes(user, route) {
        route -> Text,
        user -> Text
    }
}
