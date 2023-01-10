use crate::status_headers::StatusHeader;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_files(path: &str) -> (StatusHeader, String) {
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
