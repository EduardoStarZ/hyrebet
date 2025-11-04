use common::session::{self, LoginToken};
use common::server::Server;
use ntex::{http::HttpMessage, web};
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

#[web::post("/login")]
pub async fn login(request: web::HttpRequest, form: web::types::Form<LoginForm>, server : web::types::State<Server>) -> web::HttpResponse {

    return web::HttpResponse::Ok().finish();
}

#[web::post("/register")]
pub async fn register(request: web::HttpRequest, form: web::types::Form<RegisterForm>) -> web::HttpResponse {
   let form :  RegisterForm = form.into_inner();

    if form.password1 != form.password2 {
        return web::HttpResponse::Unauthorized().finish();
    }

    database::create_user(form.username, form.password1);

   return web::HttpResponse::Ok().finish();
}
