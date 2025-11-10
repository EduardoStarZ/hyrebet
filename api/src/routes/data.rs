use crate::database::{self, NewPost, Post};
use ntex::web;
use serde::Deserialize;
use askama::Template;

#[derive(Deserialize)]
struct PostPath {
    pub user : String,
    pub id : i32
}

#[derive(Template)]
#[template(path = "data/post.html")]
struct PostTemplate {
    pub contents : String
}

#[web::get("/{user}/{id}")]
pub async fn get_post(path : web::types::Path<PostPath>) -> web::HttpResponse {
    let post : Post = match database::get_post(&path.user, path.id) {
        Some(value) => value,
        None => return web::HttpResponse::NotFound().finish()
    };

    let template : String = PostTemplate{contents : post.contents}.render().unwrap();

    return web::HttpResponse::Ok().body(template); 
}

#[derive(Deserialize)]
struct PostForm {
    pub reply : Option<i32>,
    pub repost : Option<i32>,
    pub contents : String
    
}

#[web::post("/{user}")]
pub async fn create_post(path : web::types::Path<String>, form : web::types::Form<PostForm>) -> web::HttpResponse {
    let post : NewPost = NewPost::new(path.to_string(), form.contents.clone(), form.reply, form.repost);

    return match database::create_post(post) {
        true => web::HttpResponse::Ok().finish(),
        false => web::HttpResponse::BadRequest().finish(),
    };
}

