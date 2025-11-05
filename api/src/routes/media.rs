use ntex::web;
use askama::Template;
use serde::Deserialize;

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
