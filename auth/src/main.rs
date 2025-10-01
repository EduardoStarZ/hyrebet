use std::net::Ipv4Addr;
use common::server::{Server, ServerID};
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {

    let options : Server = Server::from(ServerID::Main, Ipv4Addr::new(127, 0, 0, 1), 3000);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .service(test)
    });

    server.bind((options.ip, options.port))?.run().await
}

#[web::get("/")]
async fn test(request : web::HttpRequest) -> web::HttpResponse {
    println!("{}", request.path());

    return web::HttpResponse::Ok().body("<h1>This is a test</h1>");
}
