use crate::database::{self, NewPost, Post};
use chrono::NaiveDateTime;
use common::session::{get_user_from_token, LoginToken};
use ntex::web;
use ntex::http::HttpMessage;
use serde::Deserialize;
use askama::Template;
use crate::{post_route_to_str, str_to_post_route};

#[derive(Deserialize)]
struct PostPath {
    pub user : String,
    pub id : i32
}

#[derive(Template)]
#[template(path = "data/post.html")]
struct PostTemplate {
    pub posts : Vec<Post>
}


struct Repost {
    pub id : i32,
    pub owner : String,
    pub contents : String,
    pub time : NaiveDateTime,
    pub repost : Option<Post>,
    pub total_likes : i32
}

#[derive(Template)]
#[template(path = "data/repost.html")]
struct RepostTemplate {
    pub posts : Vec<Repost>
}

#[derive(Template)]
#[template(path = "data/reply.html")]
struct ReplyTemplate {
    pub contents : String
}

#[web::get("/{user}/{id}")]
pub async fn get_post(path : web::types::Path<PostPath>) -> web::HttpResponse {
    let sel_post : Post = match database::get_post(&path.user, path.id) {
        Some(value) => value,
        None => return web::HttpResponse::NotFound().finish()
    };

    let template : String = match sel_post.check_type() {
        Some(value) => {
            match value.as_str() {
                "reply" => ReplyTemplate{contents : sel_post.contents}.render().unwrap(),
                "repost" => {
                    let data : (String, i32) = str_to_post_route(sel_post.repost.clone().unwrap()).unwrap();

                    let mentioned : Post = match database::get_post(&data.0, data.1) {
                       Some(value) => value,
                       None => return web::HttpResponse::NotFound().finish()
                    };

                    let repost : Repost = Repost {
                        id: sel_post.id,
                        owner: sel_post.owner.clone(),
                        total_likes: sel_post.total_likes,
                        contents : sel_post.contents.clone(),
                        repost : Some(mentioned),
                        time : sel_post.time
                        };

                    RepostTemplate{posts : vec![repost]}
                    .render()
                        .unwrap()
                },
                _ => "Null".to_string()
            }
        },
        None => PostTemplate{posts: vec![sel_post]}.render().unwrap()
    };

    return web::HttpResponse::Ok().body(template); 
}

#[derive(Deserialize)]
struct RepostForm {
    pub contents : String
}

#[derive(Deserialize)]
struct RepostPath {
    pub op_type : String,
    pub user : String,
    pub post : i32
}

#[web::post("/{op_type}/{user}/{post}")]
pub async fn create_post(path : web::types::Path<RepostPath>, form : web::types::Form<RepostForm>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    println!("Path : {} / {} / {}", path.op_type.clone(), path.user.clone(), path.post);

    println!("Form : {}", form.0.contents.clone());

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let mut new_post : NewPost = NewPost::new(user, form.0.contents.clone(), None, None);

    match path.op_type.as_str() {
        "repost" => new_post.repost = Some(post_route_to_str(path.user.clone(), path.post)),
        "reply" => new_post.reply = Some(post_route_to_str(path.user.clone(), path.post)),
        "post" => {},
        _ => return web::HttpResponse::BadRequest().finish()
    }

    return match database::create_post(new_post) {
        true => web::HttpResponse::Ok().finish(),
        false => web::HttpResponse::NotAcceptable().finish()
    }
}

#[derive(Template)]
#[template(path = "data/like_button.html")]
struct LikeButton {
    pub did_like: bool,
    pub post: Post
}

#[web::get("/like/{user}/{id}")]
pub async fn get_like(path : web::types::Path<PostPath>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let route : String = post_route_to_str(path.user.clone(), path.id);

    let liked : bool = database::liked(route.clone(), user);

    return { 
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeButton{did_like: liked, post: sel_post}.render().unwrap();
            web::HttpResponse::Ok().body(template)
    }
}

#[web::put("/like/{user}/{id}")]
pub async fn like(path : web::types::Path<PostPath>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let route : String = post_route_to_str(path.user.clone(), path.id);

    return match database::like(user, route.clone()) {
        true => { 
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeButton{did_like: true, post: sel_post}.render().unwrap();
            web::HttpResponse::Ok().body(template)
        },
        false => {
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeButton{did_like: false, post: sel_post}.render().unwrap();
            web::HttpResponse::Ok().body(template)
        }
    }
}
