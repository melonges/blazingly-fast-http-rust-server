use crate::{DEFAULT_CACHE_SIZE, DEFAULT_PORT};

pub struct Config {
    pub port: u64,
    pub cache_size: usize,
}

impl Config {
    pub fn parse_config(args: &[String]) -> Config {
        if args.len() < 2 {
            Config {
                port: DEFAULT_PORT,
                cache_size: DEFAULT_CACHE_SIZE,
            }
        } else {
            let port = args[1].parse().expect("Port must be a number");
            let cache_size = args[2].parse().expect("Cache size must be a number");
            Config { port, cache_size }
        }
    }
}
