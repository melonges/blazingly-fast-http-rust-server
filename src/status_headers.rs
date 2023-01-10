pub enum StatusHeader {
    Ok,
    NotFount,
    ServerError,
}

impl StatusHeader {
    pub fn to_string(&self) -> &str {
        match self {
            StatusHeader::Ok => "HTTP/1.1 200 OK",
            StatusHeader::NotFount => "HTTP/1.1 404 NOT FOUND",
            StatusHeader::ServerError => "HTTP/1.1 500 INTERNAL SERVER ERROR",
        }
    }
}
