use crate::cache::{self, Cache};
use crate::config::Config;
use crate::status_headers::HttpResponse;
use crate::system_utils::{read_files, FileStatus};
use std::net::TcpListener;

use std::io::BufRead;
use std::io::{BufReader, Write};
use std::net::TcpStream;
pub struct BlazingFastTcpServer {
    server: TcpListener,
}

impl BlazingFastTcpServer {
    pub fn new(config: &Config) -> BlazingFastTcpServer {
        let listener: Result<TcpListener, ()> =
            TcpListener::bind(format!("127.0.0.1:{}", config.port)).or_else(|e| {
                eprintln!("Failed to bind to port: {}", e);
                std::process::exit(1);
            });
        BlazingFastTcpServer {
            server: listener.unwrap(),
        }
    }

    pub fn run(&self, mut cache: Cache) {
        for stream in self.server.incoming() {
            match stream {
                Ok(stream) => self.connection_processing(stream, &mut cache),
                Err(_) => {
                    continue;
                }
            }
        }
    }

    pub fn connection_processing(&self, mut request: TcpStream, cache: &mut Cache) {
        let result = BufReader::new(&request).lines().next().unwrap().unwrap();
        let path = &result[5..result.len() - 9];
        let content = match cache.get(path) {
            Some(v) => HttpResponse::Ok(&v.value),
            None => {
                let data = read_files(path);
                match data {
                    FileStatus::Exist(s) => HttpResponse::Ok(cache.insert(path.to_string(), s)),
                    FileStatus::NotFount(s) => {
                        HttpResponse::NotFount(cache.insert(path.to_string(), s))
                    }
                    FileStatus::InternalError(s) => {
                        HttpResponse::ServerError(cache.insert(path.to_string(), s))
                    }
                }
            }
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            content.get_http_status_string(),
            content.get_value().len(),
            content.get_value()
        );
        request.write_all(response.as_bytes()).unwrap();
    }
}
