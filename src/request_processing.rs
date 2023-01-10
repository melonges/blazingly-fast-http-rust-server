use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::status_headers::StatusHeader;

pub fn connection_processing(mut request: TcpStream) {
    let result = BufReader::new(&request).lines().next().unwrap().unwrap();
    let path = &result[5..result.len() - 9];
    println!("{}", path);
    let content = read_files(path);
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        content.0.to_string(),
        content.1.len(),
        content.1
    );
    request.write_all(response.as_bytes()).unwrap();
}

fn read_files(path: &str) -> (StatusHeader, String) {
    let file = File::open(format!("static/{}", path));
    let mut contents = String::new();
    match file {
        Ok(file) => {
            BufReader::new(file).read_to_string(&mut contents);
            (StatusHeader::Ok, contents)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let file = File::open("static/404.html").unwrap();
                BufReader::new(file).read_to_string(&mut contents);
                (StatusHeader::NotFount, contents)
            } else {
                let file = File::open("static/500.html").unwrap();
                BufReader::new(file).read_to_string(&mut contents);
                (StatusHeader::ServerError, contents)
            }
        }
    }
}
