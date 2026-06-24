
use crate::httpr::status_code::StatusCode;
use crate::httpr::request::Request as HttpRequest;
use crate::httpr::response::HttpResponse;
use crate::httpr::http_method::HttpMethod;
use crate::server::Handler;

pub struct WebsiteHandler{
    pub path: String,
}


impl WebsiteHandler {
    pub fn new(path: String) -> Self {
        WebsiteHandler { path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.path, file_path);
        match std::fs::canonicalize(path){
            Ok(path) => {
                if path.starts_with(&self.path) {
                    std::fs::read_to_string(path).ok()
                } else {
                    println!("Attempted directory traversal attack: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: HttpRequest) -> HttpResponse {
        match request.method(){
            HttpMethod::GET => {
                match request.path() {
                    "/" => {
                        if let Some(contents) = self.read_file("index.html") {
                            HttpResponse::new(StatusCode::Ok, Some(contents))
                        } else {
                            HttpResponse::new(StatusCode::NotFound, Some("<h1>404 Not Found</h1>".to_string()))
                        }
                    },
                    "/about" => {
                        if let Some(contents) = self.read_file("about.html") {
                            HttpResponse::new(StatusCode::Ok, Some(contents))
                        } else {
                            HttpResponse::new(StatusCode::NotFound, Some("<h1>404 Not Found</h1>".to_string()))
                        }
                    },
                    path => match self.read_file(path){
                        Some(contents) => HttpResponse::new(StatusCode::Ok, Some(contents)),
                        None => HttpResponse::new(StatusCode::NotFound, Some("<h1>404 Not Found</h1>".to_string())),
                    },
                    _ => HttpResponse::new(StatusCode::NotFound, Some("<h1>404 Not Found</h1>".to_string())),
                }
            },
            _ => HttpResponse::new(StatusCode::BadRequest, Some("<h1>400 Bad Request</h1>".to_string())),
        }
    }

}