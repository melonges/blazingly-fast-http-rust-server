use crate::{config::Config, request_processing::connection_processing};
use std::net::TcpListener;
pub struct TcpServer;

impl TcpServer {
    fn new(args: Vec<String>) -> TcpListener {
        let config = Config::parse_config(&args);
        let listener: Result<TcpListener, ()> =
            TcpListener::bind(format!("127.0.0.1:{}", config.port)).or_else(|e| {
                eprintln!("Failed to bind to port: {}", e);
                std::process::exit(1);
            });
        listener.unwrap()
    }

    pub fn run(args: Vec<String>) {
        let listener = TcpServer::new(args);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => connection_processing(stream),
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
