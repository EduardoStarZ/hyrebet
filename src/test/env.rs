#[cfg(test)]
mod tests {
    use common::env;

    #[test]
    fn verify_env_args() {
        let args : Vec<String> = env::get_args();

        assert_eq!(None, args.get(1));
    }
}
