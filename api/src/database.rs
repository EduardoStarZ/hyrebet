use diesel::prelude::*;
use common::{env, database};
use crate::str_to_post_route;
use crate::{schema::posts, schema::likes};
use crate::schema::posts::dsl::*;
use crate::schema::likes::dsl::*;
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
    pub total_likes : i32,
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
    pub total_likes : i32,
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
            total_likes: 0,
            time: Local::now().naive_local()
        };
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



#[derive(Queryable, Selectable)]
#[diesel(table_name = likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Like {
    pub user : String,
    pub route : String
}

#[derive(Insertable)]
#[diesel(table_name = likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLike {
    pub user : String,
    pub route : String
}

pub fn like(liking : String, liked_route : String) -> bool {
    let mut connection = establish_connection_to_post_db();

    let like : NewLike = NewLike { user: liking.clone(), route: liked_route.clone() };

    let liked_post = likes
        .filter(user.eq(&liking).and(route.eq(&liked_route)))
        .limit(1)
        .select(Like::as_select())
        .load(&mut connection)
        .expect("Could not load database!");

    if liked_post.len() != 0 {
        return !delete_like(liking, liked_route);
    }

    return match diesel::insert_into(likes::table)
        .values(&like)
        .execute(&mut connection) {
            Ok(_) => {
                let (_, post_owner) = str_to_post_route(liked_route).unwrap();

                 _ = diesel::update(posts.filter(owner.eq(&like.user).and(id.eq(post_owner))))
                     .set(total_likes.eq(total_likes + 1))
                     .execute(&mut connection);

                    true
            },
            Err(err) => {println!("{}", err.to_string()); false}
        };

}

pub fn delete_like(liking : String, liked_route : String) -> bool {
   
    let mut connection = establish_connection_to_post_db();

    diesel::delete(likes
        .filter(route.eq(&liked_route).and(user.eq(liking))))
        .execute(&mut connection)
        .unwrap();

    let unwraped_route : (String, i32) = str_to_post_route(liked_route).unwrap();

    let liked_post : Post = get_post(&unwraped_route.0, unwraped_route.1).unwrap();

    _ = diesel::update(posts.filter(owner.eq(liked_post.owner).and(id.eq(liked_post.id))))
        .set(total_likes.eq(total_likes - 1))
        .execute(&mut connection);

    return true; 
}

pub fn get_likes(liked_route : String) -> usize {
    let mut connection = establish_connection_to_post_db();

    return match likes
        .filter(route.eq(liked_route))
        .select(Like::as_select())
        .load(&mut connection) {
            Ok(value) => value.len(),
            Err(_) => 0
        };
}

pub fn liked(liked_route : String, liking : String) -> bool {
    let mut connection = establish_connection_to_post_db();

    return match likes
        .find((&liking, &liked_route))
        .select(Like::as_select())
        .first(&mut connection)
        .optional() {
            Ok(Some(_)) => return true ,
            Ok(None) => return false,
            Err(_) => false
        };
}

pub fn get_all() -> Option<Vec<Post>> {
    let mut connection : PgConnection = establish_connection_to_post_db();

    let post = posts
        .select(Post::as_select())
        .load(&mut connection)
        .optional();

    return match post {
        Ok(value) => value,
        Err(_) => None
    };
}

