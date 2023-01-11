use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn read_files(path: &str) -> FileStatus {
    let file = File::open(format!("static/{}", path));
    let mut contents = String::new();
    match file {
        Ok(file) => {
            BufReader::new(file).read_to_string(&mut contents);
            FileStatus::Exist(contents)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let file = File::open("static/404.html").unwrap();
                BufReader::new(file).read_to_string(&mut contents);
                FileStatus::NotFount(contents)
            } else {
                let file = File::open("static/500.html").unwrap();
                BufReader::new(file).read_to_string(&mut contents);
                FileStatus::InternalError(contents)
            }
        }
    }
}

pub enum FileStatus {
    Exist(String),
    NotFount(String),
    InternalError(String),
}
