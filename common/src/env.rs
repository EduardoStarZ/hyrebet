use std::env;
use std::fs;


#[derive(Debug, PartialEq)]
pub struct EnvKeys {
    pub key: String,
    pub value: String
}

impl EnvKeys {
    pub fn new() -> EnvKeys {
        return EnvKeys { key: String::new(), value: String::new() };
    }

    pub fn from(key: &str, value: &str) -> EnvKeys {
        return EnvKeys { key: key.to_string() , value: value.to_string() };
    }

}

pub fn get_args() -> Vec<String> {
    return env::args().collect::<Vec<String>>();
}

pub fn set_hash(value: String) {
    let mut contents : String = String::from_utf8(fs::read(".env").unwrap()).unwrap();
    contents.insert_str(0, format!("AUTH_HASH={value}\n").as_str());
    
    match fs::write(".env", contents) {
        Ok(_) => (),
        Err(_) => panic!("The program could not set up the secret session string as a environment variable, shutting down...")
    } 
}

pub fn get_hash() -> Option<String> {
    match fs::metadata(".env") {
        Ok(_) => (),
        Err(_) => {
            let _ = fs::write(".env", "");
        }
    }
    
    let contents : Vec<String> = String::from_utf8(fs::read(".env").unwrap()).unwrap().split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
    
    for line in contents {
        let pairs : Vec<String> = line.split('=').map(|x| x.to_string()).collect::<Vec<String>>();

        if pairs[0] == "AUTH_HASH" {
            return Some(pairs[1].clone());
        }
    }
    return None;
}

pub fn get_value(name : &str) -> Option<String> {
    match fs::metadata(".env") {
        Ok(_) => (),
        Err(_) => {
            let _ = fs::write(".env", "");
        }
    }
    
    let contents : Vec<String> = String::from_utf8(fs::read(".env").unwrap()).unwrap().split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
    
    for line in contents {
        let pairs : Vec<String> = line.split('=').map(|x| x.to_string()).collect::<Vec<String>>();

        if pairs[0] == name {
            return Some(pairs[1].clone());
        }
    }
    return None;

}

pub fn set_value(name : &str, value : &str) {
    let mut contents : String = String::from_utf8(fs::read(".env").unwrap()).unwrap();
    contents.insert_str(0, format!("{name}={value}\n").as_str());

    match fs::write(".env", contents) {
        Ok(_) => (),
        Err(_) => panic!("Could not write the content as an environment variable")
    } 
}
