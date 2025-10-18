use common;
use std::net::Ipv4Addr;
use common::server::{Server, ServerID};
use ntex::web;
use auth::{hash, routes};
use common::middleware::SayHi;
use common::env::{self, set_hash};

#[ntex::main]
async fn main() -> std::io::Result<()> {

    match env::get_hash() {
        Some(_) => (),
        None => set_hash(hash::create_hash())
    };

    let options : Server = Server::from(ServerID::Main, Ipv4Addr::new(127, 0, 0, 1), 3000);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .service(routes::check_token)
            .service(routes::get_token)
            .wrap(SayHi)
            .state(options.clone())
    });

    server.bind((options.ip, options.port))?.run().await
}
