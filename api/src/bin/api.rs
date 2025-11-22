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
            .service(
                web::scope("/json")
                .service(data::create_post_json)
                .service(data::get_post_json)
                .service(data::like_json)
                .service(data::get_like_json)
                .service(data::profile_json)
                .service(data::home_json)

            )
            .service(media::login)
            .service(media::register)
            .service(data::create_post)
            .service(data::get_post)
            .service(data::like)
            .service(data::get_like)
            .service(data::profile)
            .service(data::home)
            .state(options.clone())
    });

    server.bind((options.ip, options.port))?.run().await
}
