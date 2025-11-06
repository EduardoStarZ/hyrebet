use diesel::prelude::*;
use common::env;
use crate::schema::files;
use crate::schema::files::dsl::*;
use rand::{rng, Rng};

#[derive(Queryable, Selectable)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct File {
    pub id : i32,
    pub name : String,
    pub owner : String
}

#[derive(Insertable)]
#[diesel(table_name = files)]
struct NewFile {
    id : i32,
    name : String,
    owner : String
}

fn establish_connection_to_file_db () -> PgConnection {
    let database_url = env::get_value("USER_DATABASE_URL").unwrap();

    return PgConnection::establish(&database_url).expect("Invalid DATABASE_URL parameter!");
}

pub fn create_file(filename : String, fileowner : String) -> bool {
    let mut connection = establish_connection_to_file_db();

    let equal_files = files
        .filter(name.eq(&filename))
        .limit(1)
        .select(File::as_select())
        .load(&mut connection)
        .expect("Could not load database!");

    if equal_files.len() > 0 {
        println!("File already exists!");
        return false;
    }

    let new_file = NewFile{name: filename, owner: fileowner, id : new_unique_id()};

    diesel::insert_into(files::table)
        .values(new_file)
        .execute(&mut connection)
        .expect("Cannot use database!");

    return true;
}

pub fn new_unique_id() -> i32 {
    let mut connection : PgConnection = establish_connection_to_file_db();

    let mut rng = rng();

    let mut file_id : i32 = rng.random();

    loop {
        let all_files = files
        .filter(id.eq(&file_id))
        .limit(1)
        .select(File::as_select())
        .load(&mut connection)
        .expect("Could not load database!");

        if !all_files.len() > 0 {
            break;
        }
    
        file_id = rng.random();
    }

    return file_id;
}
