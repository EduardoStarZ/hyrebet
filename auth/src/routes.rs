use common::session::{self, LoginToken};
use cookie::{Cookie, time::Duration};
use ntex::{http::HttpMessage,  web};
use serde::{Serialize, Deserialize};
use common::database;

#[derive(Serialize, Deserialize)]
struct LoginForm {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct RegisterForm {
    username: String,
    password1: String,
    password2: String
}

#[web::get("/login")]
pub async fn login_redirect(_request : web::HttpRequest) -> web::HttpResponse {
    return web::HttpResponse::PermanentRedirect().header("Location", "http://127.0.0.1:4000/login").finish();
}

#[web::get("/register")]
pub async fn register_redirect(_request : web::HttpRequest) -> web::HttpResponse {
    return web::HttpResponse::PermanentRedirect().header("Location", "http://127.0.0.1:4000/register").finish();
}

#[web::get("/logout")]
pub async fn logout(request : web::HttpRequest) -> web::HttpResponse {
    let cookie = request.cookie("Auth").unwrap();

    return web::HttpResponse::PermanentRedirect().del_cookie(&cookie).header("Location", "http://127.0.0.1:4000/login").finish();
}

#[web::post("/login")]
pub async fn login(_request: web::HttpRequest, form: web::types::Form<LoginForm>) -> web::HttpResponse {

    let exists_and_corr : bool = database::check_user(&form.username, &form.password);

    if !exists_and_corr {
        return web::HttpResponse::Unauthorized().finish();
    }

    let cookie = match session::create_token(session::LoginInfo{username: form.username.clone(), password: form.password.clone()}) {
        LoginToken::Value(value) => value,
        LoginToken::None => return web::HttpResponse::Unauthorized().finish()
    };

    return web::HttpResponse::PermanentRedirect().cookie(("Auth", cookie)).body("<script>location.href='http://127.0.0.1:4000/'</script>");
}

#[web::post("/login")]
pub async fn login_json(_request: web::HttpRequest, form: web::types::Json<LoginForm>) -> web::HttpResponse {

    let exists_and_corr : bool = database::check_user(&form.username, &form.password);

    if !exists_and_corr {
        return web::HttpResponse::Unauthorized().finish();
    }

    let value = match session::create_token(session::LoginInfo{username: form.username.clone(), password: form.password.clone()}) {
        LoginToken::Value(value) => value,
        LoginToken::None => return web::HttpResponse::Unauthorized().finish()
    };

    let cookie = Cookie::build(("Auth", value))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .secure(false)
        .http_only(true)
        .max_age(Duration::days(7));

    return web::HttpResponse::Ok().cookie(cookie).finish();
}

#[web::post("/register")]
pub async fn register(_request: web::HttpRequest, form: web::types::Form<RegisterForm>) -> web::HttpResponse {
   let form :  RegisterForm = form.into_inner();

    if form.password1 != form.password2 {
        return web::HttpResponse::Unauthorized().finish();
    }

    return match database::create_user(form.username, form.password1) {
        true => web::HttpResponse::PermanentRedirect().body("<script>location.href='http://127.0.0.1:4000/login'</script>"),

        false => web::HttpResponse::Unauthorized().finish()
    };
}

#[web::post("/register")]
pub async fn register_json(_request: web::HttpRequest, form: web::types::Json<RegisterForm>) -> web::HttpResponse {
   let form :  RegisterForm = form.into_inner();

    if form.password1 != form.password2 {
        return web::HttpResponse::Unauthorized().finish();
    }

    let value = "{value : true}";

    return match database::create_user(form.username, form.password1) {
        true => web::HttpResponse::Ok().json(&value),

        false => web::HttpResponse::Unauthorized().finish()
    };
}

#[web::get("/check")]
pub async fn check_json(request: web::HttpRequest) -> web::HttpResponse {
    return match request.cookie("Auth") {
        Some(_) => web::HttpResponse::Ok().finish(),
        None => web::HttpResponse::Unauthorized().finish()
    }
}
