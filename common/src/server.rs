use std::net::{Ipv4Addr};

pub struct Server {
    pub id : u8,
    pub name : String,
    pub ip : Ipv4Addr,
    pub port : u16,
}
