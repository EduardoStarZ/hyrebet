use std::net::{Ipv4Addr};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ServerID {
    Unknown,
    Main,
    Auth,
    Static
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Server {
    pub id : ServerID,
    pub ip : Ipv4Addr,
    pub port : u16,
}

impl Server {
    pub fn new() -> Server {
        return Server { 
            id : ServerID::Unknown,
            ip : Ipv4Addr::new(127, 0, 0, 1),
            port : 8080
        };
    }

    pub fn from(id: ServerID, ip: Ipv4Addr, port: u16) -> Server {
        return Server {id, ip, port};
    }
}
