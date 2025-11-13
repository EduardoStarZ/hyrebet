use diesel::prelude::*;
use common::{env, database};
use crate::schema::posts;
use crate::schema::posts::dsl::*;
use rand::{rng, Rng};
use chrono::{Local, NaiveDateTime};

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id : i32,
    pub reply : Option<String>,
    pub repost : Option<String>,
    pub owner : String,
    pub contents : String,
    pub likes : i32,
    pub time : NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id : i32,
    pub reply : Option<String>,
    pub repost : Option<String>,
    pub owner : String,
    pub contents : String,
    pub likes : i32,
    pub time : NaiveDateTime
}

impl NewPost {
    pub fn new(p_owner : String, p_contents : String, p_reply : Option<String>, p_repost : Option<String>) -> NewPost {
        return NewPost { 
            id: new_unique_id(&p_owner),
            reply : p_reply,
            repost: p_repost,
            owner: p_owner,
            contents: p_contents,
            likes: 0,
            time: Local::now().naive_local()
        };
    }
}

impl Post {
    pub fn check_type(&self) -> Option<String> {
        if self.reply.is_some() {
            return Some(String::from("reply"));
        } 

        if self.repost.is_some() {
            return Some(String::from("repost"));
        }

        return None;
    }
}

fn establish_connection_to_post_db () -> PgConnection {
    let database_url = env::get_value("POST_DATABASE_URL").unwrap();

    return PgConnection::establish(&database_url).expect("Invalid DATABASE_URL parameter!");
}

pub fn create_post(mut post : NewPost) -> bool {
    let mut connection = establish_connection_to_post_db();

    if !database::exists(&post.owner) {
        return false;
    }

    let equal_posts = posts
        .filter(id.eq(&post.id))
        .limit(1)
        .select(Post::as_select())
        .load(&mut connection)
        .expect("Could not load database!");

    if equal_posts.len() > 0 {
        post.id = new_unique_id(&post.owner);
    }

    return match diesel::insert_into(posts::table)
        .values(post)
        .execute(&mut connection) {
            Ok(_) => true,
            Err(_) => false
        };
}

pub fn new_unique_id(username : &String) -> i32 {
    let mut connection : PgConnection = establish_connection_to_post_db();

    let mut rng = rng();

    let mut post_id : i32 = rng.random();

    loop {
        let all_posts = posts
        .filter(id.eq(&post_id).and(owner.eq(username)))
        .limit(1)
        .select(Post::as_select())
        .load(&mut connection)
        .expect("Could not load database!");

        if !all_posts.len() > 0 {
            break;
        }
    
        post_id = rng.random();
    }

    return post_id;
}

pub fn get_post(post_username : &String, post_id : i32) -> Option<Post> {
    let mut connection : PgConnection = establish_connection_to_post_db();

    let post = posts
        .find((post_id, post_username))
        .select(Post::as_select())
        .first(&mut connection)
        .optional();

    return match post {
        Ok(value) => value,
        Err(_) => None
    };
}
