use std::net::TcpListener;
use std::io::Read;
use crate::httpr::request::Request as HttpRequest;
use crate::httpr::response::HttpResponse;
use crate::httpr::status_code::StatusCode;
use std::convert::TryFrom;
use std::io::Write;


pub trait Handler {
    fn handle_request(&mut self, request: HttpRequest) -> HttpResponse;
    fn handle_bad_request(&mut self, e: &dyn std::error::Error) -> HttpResponse {
        println!("Failed to parse request: {}", e);
        HttpResponse::new(StatusCode::BadRequest, Some("Bad Request".to_string()))
    }
}

pub struct Server {
    pub address: String,
    pub port: u16,
}

impl Server {
    pub fn new(address: &str) -> Self {
        let parts: Vec<&str> = address.split(':').collect();
        let address = parts[0].to_string();
        let port = parts[1].parse::<u16>().unwrap_or(8080);
        Server { address, port }
    }

    pub fn run(&self , handler: &mut impl Handler) {
        println!("Server running on {}:{}", self.address, self.port);
        // Here you would add code to start the server and handle requests
        let listner = TcpListener::bind(format!("{}:{}", self.address, self.port)).expect("Could not bind to address");

        loop {
            match listner.accept() {
                Ok((mut stream, addr)) => {
                    println!("New connection from {}", addr);
                    // Here you would handle the incoming request
                    let mut buffer = [0; 512]; // You would typically read the request into a buffer
                    match stream.read(&mut buffer) {
                        Ok(0) =>{
                            println!("Connection closed by client: {}", addr);
                            continue;
                        },
                        Ok(request) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer[..request]));
                            // Here you would parse the request and send a response
                            let _ = match HttpRequest::try_from(&buffer[..request]) {
                                Ok(req) => { 
                                    println!("Parsed request: {:?}", req);
                                    handler.handle_request(req);
                                    let response = HttpResponse::new(StatusCode::Ok, Some(" <h1> It Works!</h1> ".to_string()));
                                    response.send(&mut stream).expect("Failed to send response");
                                },
                                Err(e) => {
                                    let response = handler.handle_bad_request(&e);
                                    eprintln!("Failed to parse request: {}", e);
                                    let response = HttpResponse::new(StatusCode::BadRequest, Some("Bad Request".to_string())); 
                                    response.send(&mut stream).expect("Failed to send response");
                                    continue;
                                }
                            };

                        }
                        Err(e) => {
                            eprintln!("Error reading from stream: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }    
        }
    }
}
