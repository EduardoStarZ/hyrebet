use ntex::web;
use askama::Template;
use serde::Deserialize;
use crate::{database::Post, Base};

#[web::get("/login")]
pub async fn login() -> web::HttpResponse {
    #[derive(Template, Deserialize)]
    #[template(path="auth/login.html")]
    struct LoginTemplate;

    let template : String = match LoginTemplate.render() {
        Ok(value) => value,
        Err(_) => return web::HttpResponse::InternalServerError().finish()
    };

    return web::HttpResponse::Ok().body(template);
}


#[web::get("/register")]
pub async fn register() -> web::HttpResponse {
    #[derive(Template, Deserialize)]
    #[template(path = "auth/register.html")]
    struct RegisterTemplate;

    let template : String = match RegisterTemplate.render() {
        Ok(value) => value,
        Err(_) => return web::HttpResponse::InternalServerError().finish()
    };

    return web::HttpResponse::Ok().body(template);
}

#[derive(Template)]
#[template(path = "data/post.html")]
struct Test {
    pub posts : Vec<Post>
}

#[web::get("/test")]
pub async fn test() -> web::HttpResponse {
    let all : Vec<Post> = crate::database::get_all().unwrap();

    let post : Test = Test{posts : all};

    let template : Base = Base {title: String::from("Test"), scripts: None, body: post.render().unwrap()};

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
