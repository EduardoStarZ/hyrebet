#[cfg(test)]
mod tests {
    use common::env::{self, EnvKeys};

    #[test]
    fn test_env_writing() {
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
        let expected : Vec<EnvKeys> = Vec::from(EnvKeys::from("test", "true"));
        let contents : Vec<EnvKeys> = env::read_env(".testenv");
    
        assert_eq!(expected, contents); 
    }
}
