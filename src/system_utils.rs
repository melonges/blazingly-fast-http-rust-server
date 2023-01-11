use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn read_files(path: &str) -> FileStatus {
    if path == "/" || path.is_empty() {
        let mut contents = String::new();
        BufReader::new(File::open("static/index.html").unwrap())
            .read_to_string(&mut contents)
            .unwrap();
        return FileStatus::Exist(contents);
    }
    let file = File::open(format!("static/{}", path));
    let mut contents = String::new();
    match file {
        Ok(file) => {
            BufReader::new(file).read_to_string(&mut contents).unwrap();
            FileStatus::Exist(contents)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let file = File::open("static/404.html").unwrap();
                BufReader::new(file).read_to_string(&mut contents).unwrap();
                FileStatus::NotFount(contents)
            } else {
                let file = File::open("static/500.html").unwrap();
                BufReader::new(file).read_to_string(&mut contents).unwrap();
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
