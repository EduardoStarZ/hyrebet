use crate::database::{self, NewPost, Post};
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
    pub contents : String
}


#[derive(Template)]
#[template(path = "data/repost.html")]
struct RepostTemplate {
    pub op_contents : String,
    pub contents : String
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
                    let data : (String, i32) = str_to_post_route(sel_post.repost.unwrap()).unwrap();

                    let mentioned : Post = match database::get_post(&data.0, data.1) {
                       Some(value) => value,
                       None => return web::HttpResponse::NotFound().finish()
                    };

                    RepostTemplate{op_contents: mentioned.contents, contents : sel_post.contents}
                    .render()
                        .unwrap()
                },
                _ => "Null".to_string()
            }
        },
        None => PostTemplate{contents: sel_post.contents}.render().unwrap()
    };

    return web::HttpResponse::Ok().body(template); 
}

#[derive(Deserialize)]
struct RepostForm {
    pub contents : String,
}

#[derive(Deserialize)]
struct RepostPath {
    pub op_type : String,
    pub user : String,
    pub post : i32
}

#[web::post("/{op_type}/{user}/{post}")]
pub async fn create_post(path : web::types::Path<RepostPath>, form : web::types::Form<RepostForm>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let mut new_post : NewPost = NewPost::new(user, form.contents.clone(), None, None);

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

#[web::put("/like/{user}/{id}")]
pub async fn like(path : web::types::Path<PostPath>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    return match database::like(user, post_route_to_str(path.user.clone(), path.id)) {
        true => web::HttpResponse::Ok().finish(),
        false => web::HttpResponse::NotAcceptable().finish()
    }
}
