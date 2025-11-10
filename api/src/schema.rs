diesel::table!{
    posts(id, owner) {
        id -> Integer,
        reply -> Nullable<Integer>,
        repost -> Nullable<Integer>,
        owner -> VarChar,
        contents -> Text,
        likes -> Integer,
        time -> Timestamptz
    }
}
