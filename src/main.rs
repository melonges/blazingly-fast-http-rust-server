use tcp_server::BlazingFastTcpServer;

const DEFAULT_PORT: u64 = 7878;
const DEFAULT_CACHE_SIZE: u64 = 1000;

mod cache;
mod config;
mod init;
mod request_processing;
mod status_headers;
mod system_utils;
mod tcp_server;

fn main() {
    init::init();
    let config = config::Config::parse_config(std::env::args().collect());
    let super_fast_tcp_server = BlazingFastTcpServer::new(config);
    super_fast_tcp_server.run();
}
