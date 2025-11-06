diesel::table!{
    files(id) {
        id -> Integer,
        name -> Text,
        owner -> VarChar,
    }
}
