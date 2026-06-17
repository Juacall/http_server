mod server {
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

        pub fn run(&self) {
            println!("Server running on {}:{}", self.address, self.port);
            // Here you would add code to start the server and handle requests
        }
    }
}

fn main() {
    println!("Hello, world!");
    let server = server::Server::new("localhost:8080 ");
    server.run();
}

struct Request {
    path: String,
    query_string: Option<String>, // Optional query string for GET requests
    method: HttpMethod,
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}