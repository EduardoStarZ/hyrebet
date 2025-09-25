use std::net::Ipv4Addr;
use common::server::Server;
use ntex::web;
use ntex_files as ntxfs;

#[ntex::main]
async fn main() -> std::io::Result<()> {

    let options : Server = Server::from(0, "Auth", Ipv4Addr::new(127, 0, 0, 1), 8080);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .service(ntxfs::Files::new("/", "../static/").show_files_listing())
    });

    server.bind((options.ip, options.port))?.run().await
}
