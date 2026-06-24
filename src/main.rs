use std::env;

mod server;
mod httpr;
mod website_handler;



fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or_else(|_| default_path);
    println!("Using public path: {}", public_path);
    let server = server::Server::new("localhost:8080 ");
    let mut handler = website_handler::WebsiteHandler::new(public_path);
    server.run(&mut handler );
}