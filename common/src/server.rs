use std::net::{Ipv4Addr};

pub struct Server {
    pub id : u8,
    pub name : &'static str,
    pub ip : Ipv4Addr,
    pub port : u16,
}

impl Server {
    pub fn new() -> Server {
        return Server { 
            id : 0,
            name : "Server",
            ip : Ipv4Addr::new(127, 0, 0, 1),
            port : 8080
        };
    }

    pub fn from(id: u8, name: &'static str, ip: Ipv4Addr, port: u16) -> Server {
        return Server {id, name, ip, port};
    }

}
