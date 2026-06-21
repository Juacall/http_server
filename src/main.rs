mod server;
mod httpr;



fn main() {
    println!("Hello, world!");
    let server = server::Server::new("localhost:8080 ");
    server.run();
}