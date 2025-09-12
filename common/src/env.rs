use std::env;
use std::fs;


#[derive(Debug, PartialEq)]
pub struct EnvKeys {
    pub key: String,
    pub value: String
}

impl EnvKeys {
    fn new() -> EnvKeys {
        return EnvKeys { key: String::new(), value: String::new() };
    }

    fn from(key: &str, value: &str) -> EnvKeys {
        return EnvKeys { key: key.to_string() , value: value.to_string() };
    }
}

pub fn get_args() -> Vec<String> {
    return env::args().collect::<Vec<String>>();
}

pub fn read_env(file : &str) -> Vec<EnvKeys> {
    let contents : String = match fs::read_to_string(file) {
        Ok(value) => value,
        Err(_) => panic!("Provided file either does not exist or does not contain valid UTF-8!")
    };

    let mut reader : Vec<EnvKeys> = Vec::new();

    for line in contents.split("\n") {

        println!("{line}");

        let try_spread = line.split_once("=");

        let spread : (&str, &str) = match try_spread {
            Some(value) => value,
            None => panic!("Given environment does not follow required format!")
        };

        reader.push(EnvKeys::from(spread.0, spread.1));

    }

    return reader;
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
