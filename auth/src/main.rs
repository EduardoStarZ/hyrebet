use std::net::Ipv4Addr;
use common::server::{Server, ServerID};
use ntex::web;
use auth::routes;

#[ntex::main]
async fn main() -> std::io::Result<()> {

    let options : Server = Server::from(ServerID::Main, Ipv4Addr::new(127, 0, 0, 1), 3000);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .service(routes::check_token)
            .service(routes::get_token)
    });

    server.bind((options.ip, options.port))?.run().await
}
