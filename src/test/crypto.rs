
#[cfg(test)]
mod tests {
    use common::crypto::hash;
    
    #[test]
    fn encrypt() {
        let info : String = String::from("bolo");
        let expected_hash : String = String::from("bcaeb187660220691e3f616c5313eb5b8a78f6ee11cb00649c74697240a86f4c");
        let hashed_info : String = hash(info);

        assert_eq!(hashed_info, expected_hash);
    }
}
