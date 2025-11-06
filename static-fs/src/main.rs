use std::net::Ipv4Addr;
use common::server::{Server, ServerID};
use ntex::web;
use ntex_files as ntxfs;
use serde::Deserialize;

#[ntex::main]
async fn main() -> std::io::Result<()> {

    let options : Server = Server::from(ServerID::Static, Ipv4Addr::new(127, 0, 0, 1), 8080);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .service(ntxfs::Files::new("/", "../static/").show_files_listing())
    });

    server.bind((options.ip, options.port))?.run().await
}

#[derive(Deserialize)]
struct FileForm {
    pub name : String,
    pub extension : String,
    pub destination : String,
    pub contents : String
}

#[web::post("/send")]
async fn get_file(form : web::types::Form<FileForm>) -> web::HttpResponse {
   let file : String = format!("{}.{}", form.name.clone(), form.extension.clone());

   let destination : String = form.destination.clone();

   let str_contents : String = form.contents.clone();

   let contents : Vec<u8> = hex_to_u8(&str_contents);

    return web::HttpResponse::Ok().finish();
}

fn hex_to_u8(hex : &String) -> Vec<u8> {
    let mut hexes : Vec<u8> = Vec::new();
    let mut copied_str = hex.chars().rev().collect::<String>();

    for _ in 0..(hex.len() / 2) {
        let sub_str : String = format!("{}{}", copied_str.pop().unwrap(), copied_str.pop().unwrap());

        let value : u8 = match u8::from_str_radix(sub_str.as_str(), 16) {
            Ok(value) => value,
            Err(err) => {
                println!("Error while parsing a hexadecimal value to a unsigned integer of 8 bits! Returning a u8::MIN value and displaying backtrace: {}", err.to_string()); 
                u8::MIN 
            }
        };
        hexes.push(value);
    }
    return hexes;
}

