
use crate::httpr::status_code::StatusCode;
use std::io::Write;

pub struct HttpResponse {
    pub status_code: StatusCode,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        HttpResponse { status_code, body }
    }

    pub fn to_string(&self) -> String {
        let status_line = match self.status_code {
            StatusCode::Ok => "HTTP/1.1 200 OK\r\n",
            StatusCode::BadRequest => "HTTP/1.1 400 Bad Request\r\n",
            StatusCode::NotFound => "HTTP/1.1 404 Not Found\r\n",
            StatusCode::InternalServerError => "HTTP/1.1 500 Internal Server Error\r\n",
        };

        let body = self.body.as_ref().map_or("", |b| b.as_str());
        format!("{}Content-Length: {}\r\n\r\n{}", self.status_code, status_line,body)
    }


    pub fn send(&self, stream: &mut std::net::TcpStream) -> std::io::Result<()> {
        let response_string =  match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), response_string)?;
        Ok(())
    }
}