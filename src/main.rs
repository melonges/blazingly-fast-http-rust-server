const DEFAULT_PORT: u32 = 7878;
const DEFAULT_CACHE_SIZE: u32 = 1000;

mod cache;
mod config;
mod init;
mod request_processing;
mod status_headers;
mod tcp_server;

fn main() {
    init::init();
    let args: Vec<String> = std::env::args().collect();
    tcp_server::TcpServer::run(args);
}
