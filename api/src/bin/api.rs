use std::net::Ipv4Addr;
use ntex::{self, web};
use common::server::{Server, ServerID};
use common::middleware::CheckLogin;
use api::routes::*;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let options : Server = Server::from(ServerID::Main, Ipv4Addr::new(127, 0, 0, 1), 4000);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .wrap(CheckLogin)
            .service(media::login)
            .service(media::register)
            .service(data::create_post)
            .service(data::get_post)
            .service(data::like)
            .service(data::get_like)
            .service(media::test)
            .state(options.clone())
    });

    server.bind((options.ip, options.port))?.run().await
}
