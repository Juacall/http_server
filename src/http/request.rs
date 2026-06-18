pub struct Request {
    pub path: String,
    pub query_string: Option<String>, // Optional query string for GET requests
    pub method: HttpMethod,
}
