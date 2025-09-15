#[cfg(test)]
mod tests {
    use common::env::{self, EnvKeys};
    use std::fs;

    #[test]
    fn test_env_writing() {
        let _ = fs::File::create(".testenv");

        let contents : &str = "test=true";

        env::append_env(".testenv", contents);
        
    }

    #[test]
    fn verify_env_args() {
        let args : Vec<String> = env::get_args();

        assert_eq!(None, args.get(1));
    }

    #[test]
    fn read_env() {
        let expected : EnvKeys = EnvKeys::from("test", "true");
        let bind : Vec<EnvKeys> = env::read_env(".testenv");
        let contents : &EnvKeys = bind.get(0).unwrap();
    
        println!("{} | {} / {} | {}", expected.key, expected.value, contents.key, contents.value);

        assert_eq!((expected.key == contents.key), (expected.value == contents.value)); 
        
    }

    #[test]
    fn test_env() {
        let desired_key : &str = "test";

        let key = env::search_env("test", ".testenv").unwrap();
    
        assert_eq!(desired_key, key.key.as_str());
    }
}
