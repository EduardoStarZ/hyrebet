use common::session::{self, LoginToken};
use common::server::Server;
use cookie::Cookie;
use ntex::{http::HttpMessage, web};
use reqwest;
use serde::{Serialize, Deserialize};
use common::database;

#[derive(Serialize, Deserialize)]
struct UserForm {
    username: String,
    password: String
}

pub fn get_token(username : &String, password : &String) -> String {
    let token = session::create_token(session::LoginInfo {
        username: username.clone(),
        password: password.clone()
    })
    .0
    .unwrap();

    return token;
}

pub async fn check_token(cookie: Option<String>) -> web::HttpResponse {
    if !session::check_token_val(&LoginToken(cookie)) {
        return web::HttpResponse::Unauthorized().finish();
    }

    return web::HttpResponse::Ok().finish();
}

#[web::post("/login")]
pub async fn login(request: web::HttpRequest, form: web::types::Form<UserForm>, server : web::types::State<Server>) -> web::HttpResponse {

    let cookie = match request.cookie("Auth") {
        Some(value) => return web::HttpResponse::Ok().finish(),
        None => ()
    };

    get_token(&form.username, &form.password);    

    let route : String = format!("http://{}:{}/get-token", server.ip, server.port);
    
    let form_data = [("username", form.username.clone()), ("password", form.password.clone())];

    let response = match reqwest::Client::new().post(route).form(&form_data).send().await {
        Ok(resp) => resp,
        Err(err) => return web::HttpResponse::BadRequest().finish()
    };

    return web::HttpResponse::Ok().finish();
}
