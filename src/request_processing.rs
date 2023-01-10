
use crate::system_utils::read_files;

use std::io::BufRead;
use std::io::{BufReader, Write};
use std::net::TcpStream;

pub fn connection_processing(mut request: TcpStream) {
    let result = BufReader::new(&request).lines().next().unwrap().unwrap();
    let path = &result[5..result.len() - 9];
    let content = read_files(path);
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        content.0.to_string(),
        content.1.len(),
        content.1
    );
    request.write_all(response.as_bytes()).unwrap();
}
