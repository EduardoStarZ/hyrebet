use std::env;

pub fn get_args() -> Vec<String> {
    return env::args().collect::<Vec<String>>();
}
