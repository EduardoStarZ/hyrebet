use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::env;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub name : String,
    pub join_date : NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    name : String,
    join_date : NaiveDateTime
}

fn establish_connection_to_user_db () -> PgConnection {
    let database_url = env::get_value("DATABASE_URL").unwrap();

    return PgConnection::establish(&database_url).expect("Invalid DATABASE_URL parameter!");
}

pub fn create_user(username : String) {
    let mut connection = establish_connection_to_user_db();

    let equal_usernames = users
        .filter(name.eq(&username))
        .limit(1)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Could not load database!");

    if equal_usernames.len() > 0 {
        println!("Username already exists!");
        return;
    }

    let new_user = NewUser{name: username, join_date: Local::now().naive_local()};

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&mut connection)
        .expect("Cannot use database!");
}
