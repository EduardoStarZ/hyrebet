use ntex::{http::HttpMessage, web};
use crate::session::{self, LoginToken};

#[web::get("/get-token")]
pub async fn get_token() -> web::HttpResponse {
    let token = session::create_token(session::LoginInfo{username : "memememe".to_string(), password: "asdasdasd".to_string()}, "test hash").0.unwrap();

    return web::HttpResponse::PermanentRedirect().cookie(("Auth", token)).finish();
}

    #[web::get("/check-token")]
pub async fn check_token(request : web::HttpRequest) -> web::HttpResponse {
    let cookie = match request.cookie("Auth") {
        Some(value) => value,
        None => return web::HttpResponse::BadRequest().finish()
    };

    if !session::check_token_val(&LoginToken(Some(cookie.value().to_string())), "test hash") {
        return web::HttpResponse::Unauthorized().finish();
    }

    return web::HttpResponse::Ok().finish();
}
