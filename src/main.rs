use cache::Cache;
use tcp_server::BlazingFastTcpServer;

const DEFAULT_PORT: u64 = 7878;
const DEFAULT_CACHE_SIZE: usize = 1000;
const IP: &str = "127.0.0.1";
mod cache;
mod config;
mod init;
mod status_headers;
mod system_utils;
mod tcp_server;

fn main() {
    init::init();
    let args: String = std::env::args().collect();
    let config = config::Config::parse_config(&[args]);
    let super_fast_tcp_server = BlazingFastTcpServer::new(&config);
    let cache = Cache::new(&config);
    println!("started on http://{IP}:{}", config.port);
    super_fast_tcp_server.run(cache);
}
