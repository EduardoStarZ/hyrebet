use crate::database::{self, Post};
use ntex::web;
use serde::Deserialize;
use common::database as common_database;

#[derive(Deserialize)]
struct PostPath {
    pub user : String,
    pub id : i32
}

#[web::get("/{user}/{id}")]
pub async fn post(path : web::types::Path<PostPath>) -> web::HttpResponse {
    let post : Post = match database::get_post(path.id) {
        Some(value) => value,
        None => return web::HttpResponse::NotFound().finish()
    };

    return web::HttpResponse::Ok().finish(); 
}
