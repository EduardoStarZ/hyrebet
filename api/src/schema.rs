diesel::table!{
    posts(id, owner) {
        id -> Integer,
        reply -> Nullable<VarChar>,
        repost -> Nullable<VarChar>,
        owner -> VarChar,
        contents -> Text,
        likes -> Integer,
        time -> Timestamptz
    }
}
