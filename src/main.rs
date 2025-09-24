use common::env::read_env;
use common::server::Server;
use ntex::web::{self, service};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let server = web::HttpServer::new(move || {
        web::App::new()
            
    });

    server.bind(("127.0.0.1", 8080))?
    .run()
    .await
}
