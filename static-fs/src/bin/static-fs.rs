use std::{fs::{self, File}, io::Write, net::Ipv4Addr};
use common::server::{Server, ServerID};
use ntex::{web, http};
use ntex_files as ntxfs;
use serde::Deserialize;
use ntex_cors::Cors;

#[ntex::main]
async fn main() -> std::io::Result<()> {

    let dir : &str = "./static/";

    match fs::create_dir(dir) {
        Ok(_) => (),
        Err(_) => println!("Static directory already exists, ignoring...")
    };

    let options : Server = Server::from(ServerID::Static, Ipv4Addr::new(127, 0, 0, 1), 8080);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .service(ntxfs::Files::new("/", "./static/").show_files_listing())
            .wrap(
              Cors::new() // <- Construct CORS middleware builder
              .allowed_origin("http://127.0.0.1:5000")
              .allowed_origin("http://127.0.0.1:4000")
              .allowed_methods(vec![http::Method::GET, http::Method::POST, http::Method::PUT])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(36000)
              .finish())
    });

    server.bind((options.ip, options.port))?.run().await
}
