use ntex::web;
use askama::Template;
use serde::Deserialize;
use crate::{routes::data::{PostTemplate, PostWrapper}, Base};

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


#[web::get("/test")]
pub async fn test() -> web::HttpResponse {
    let all : Vec<PostWrapper> = crate::database::get_all().unwrap().iter().map(|post| PostWrapper::new(&post)).collect::<Vec<PostWrapper>>();

    let post : PostTemplate = PostTemplate{posts : all};

    let template : Base = Base {title: String::from("Test"), scripts: None, body: post.render().unwrap()};

    return web::HttpResponse::Ok().body(template.render().unwrap());
}
