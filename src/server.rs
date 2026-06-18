use TCPListener;

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
        let listner = TCPListener::bind(format!("{}:{}", self.address, self.port)).unwrap();

        loop {
            match listner.accept() {
                Ok((stream, addr)) => {
                    println!("New connection from {}", addr);
                    // Here you would handle the incoming request
                    match stream.read_to_string() {
                        Ok(request) => {
                            println!("Received request: {}", request);
                            // Here you would parse the request and send a response
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
