pub enum HttpResponse<'a> {
    Ok(&'a String),
    NotFount(&'a String),
    ServerError(&'a String),
}

impl<'a> HttpResponse<'a> {
    pub fn get_http_status_string(&self) -> &'static str {
        match self {
            HttpResponse::Ok(_) => "HTTP/1.1 200 OK",
            HttpResponse::NotFount(_) => "HTTP/1.1 404 NOT FOUND",
            HttpResponse::ServerError(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR",
        }
    }
    pub fn get_value(&self) -> &'a String {
        match self {
            HttpResponse::Ok(s) => s,
            HttpResponse::NotFount(s) => s,
            HttpResponse::ServerError(s) => s,
        }
    }
}
