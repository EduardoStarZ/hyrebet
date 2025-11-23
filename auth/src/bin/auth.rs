use common;
use std::net::Ipv4Addr;
use common::server::{Server, ServerID};
use ntex::web;
use auth::{hash, routes};
use common::middleware::CheckLogin;
use common::env::{self, set_hash};

#[ntex::main]
async fn main() -> std::io::Result<()> {

    match env::get_hash() {
        Some(_) => (),
        None => set_hash(hash::create_hash())
    };

    let options : Server = Server::from(ServerID::Auth, Ipv4Addr::new(127, 0, 0, 1), 3000);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .wrap(CheckLogin)
            .service(
                web::scope("/json")
                .service(routes::login_json)
                .service(routes::register_json)
                .service(routes::check_json)
            )
            .service(routes::login)
            .service(routes::register)
            .service(routes::login_redirect)
            .service(routes::register_redirect)
            .service(routes::logout)
            .state(options.clone())
    });

    server.bind((options.ip, options.port))?.run().await
}
