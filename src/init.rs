use std::{path::Path, process};

pub fn check_default_files(current_path: &Path) {
    if !(Path::new("static/404.html").exists()
        && Path::new("static/500.html").exists()
        && Path::new("static/index.html").exists())
    {
        eprintln!("Error: 404.html or 500.html not found");
        process::exit(1);
    }
}
