use crate::{config::Config, request_processing::connection_processing};
use std::net::TcpListener;
pub struct BlazingFastTcpServer {
    server: TcpListener,
}

impl BlazingFastTcpServer {
    pub fn new(config: Config) -> BlazingFastTcpServer {
        let listener: Result<TcpListener, ()> =
            TcpListener::bind(format!("127.0.0.1:{}", config.port)).or_else(|e| {
                eprintln!("Failed to bind to port: {}", e);
                std::process::exit(1);
            });
        BlazingFastTcpServer {
            server: listener.unwrap(),
        }
    }

    pub fn run(&self) {
        for stream in self.server.incoming() {
            match stream {
                Ok(stream) => connection_processing(stream),
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
