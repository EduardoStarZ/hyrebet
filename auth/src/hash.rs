use rand::{Rng, distr::Alphanumeric};


pub fn create_hash() -> String {
    let s = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect::<String>();

    return s;
}
