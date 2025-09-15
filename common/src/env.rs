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

pub fn read_env(file : &str) -> Vec<EnvKeys> {
    let byte_contents : Vec<u8> = match fs::read(file) {
        Ok(value) => value,
        Err(err) => panic!("{err}")
    };

    let contents : &str = match str::from_utf8(&(*byte_contents)) {
        Ok(value) => value,
        Err(err) => panic!("{err}")
    };

    let sliced : Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut result : Vec<EnvKeys> = Vec::new();

    for item in sliced {

        if !item.trim().is_empty() {

            let binds : [&str;2] = match item
                .split("=")
                .collect::<Vec<&str>>()
                .first_chunk::<2>() {
                    Some(value) => *value,
                    None => panic!("Key has invalid signature")
                };

            result.push(EnvKeys::from(binds[0], binds[1])); 
        }
    }

    return result;
}

pub fn append_env(filename: &str, contents: &str) {
    let existing : String = match fs::read_to_string(filename) {
        Ok(value) => value,
        Err(_) => panic!("File with provided name does not exist!")
    };

    let appended : String = format!("{existing}\n{contents}");

    match fs::write(filename, appended) {
        Ok(_) => return,
        Err(_) => panic!("Could not read to file! Program does not have enough permission or file doesn't exist!")
    };


}
