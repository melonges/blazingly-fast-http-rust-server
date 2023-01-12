use std::path::PathBuf;

use crate::{DEFAULT_CACHE_SIZE, DEFAULT_PORT};

pub struct Config {
    pub port: u64,
    pub cache_size: usize,
    pub path: PathBuf,
}

impl Config {
    pub fn build() -> Config {
        let args: Vec<String> = std::env::args().collect();
        let mut port = None;
        let mut cache_size = None;
        let mut path = None;

        for i in 1..args.len() {
            match args[i].as_ref() {
                "-p" | "--port" => {
                    if i + 1 < args.len() {
                        port = Some(args[i + 1].parse().expect("Port must be a number"));
                    }
                }
                "-c" | "--cache-size" => {
                    if i + 1 < args.len() {
                        cache_size =
                            Some(args[i + 1].parse().expect("Cache size must be a number"));
                    }
                }
                "-path" | "--path" => {
                    if i + 1 < args.len() {
                        path = Some(PathBuf::from(&args[i + 1]));
                    }
                }

                _ => (),
            }
        }

        let final_config = Config {
            port: port.unwrap_or(DEFAULT_PORT),
            cache_size: cache_size.unwrap_or(DEFAULT_CACHE_SIZE),
            path: path.unwrap_or(std::env::current_dir().unwrap()),
        };
        return final_config;
    }
}
