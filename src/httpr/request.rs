use crate::httpr::http_method::{self, HttpMethod, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path;
use std::str::Utf8Error;

pub struct Request {
    pub path: String,
    pub query_string: Option<String>, // Optional query string for GET requests
    pub method: HttpMethod,
}

impl Request {


    pub fn new(path: String, method: HttpMethod) -> Self {
        Request {
            path,
            query_string: None,
            method,
        }
    }
}

fn getNextWord(request: &str) -> Option<(&str, &str)> {
    let trimmed_request = request.trim_start();
    if trimmed_request.is_empty() {
        return None;
    }

    let mut chars = trimmed_request.chars();
    let mut word_end_index = 0;

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            break;
        }
        word_end_index += c.len_utf8();
    }

    let word = &trimmed_request[..word_end_index];
    let remaining_request = &trimmed_request[word_end_index..];

    Some((word, remaining_request))
}
impl TryFrom<&[u8]> for Request {
    type Error = RequestParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = std::str::from_utf8(buffer).map_err(|_| RequestParseError::InvalidEncoding)?;
        let (method_str, remaining_request) = getNextWord(request).ok_or(RequestParseError::InvalidRequest)?;
        let (path_str, remaining_request) = getNextWord(remaining_request).ok_or(RequestParseError::InvalidRequest)?;
        let (protocol_str, _) = getNextWord(remaining_request).ok_or(RequestParseError::InvalidRequest)?;

        if protocol_str != "HTTP/1.1" {
            return Err(RequestParseError::InvalidRequest);
        }

        let method: HttpMethod = method_str.parse()?;

        let mut query_string = None;
        let mut path = path_str;

        if path.is_empty() || !path.starts_with('/') {
            return Err(RequestParseError::InvalidPath);
        }

        if let Some(index) = path_str.find('?') {
            path = &path_str[..index];
            let query = &path_str[index + 1..];
            if query.is_empty() {
                return Err(RequestParseError::InvalidQueryString);
            }
            query_string = Some(query.to_string());
        }

        Ok(Self {
            path: path.to_string(),
            query_string,
            method,
        })
    }
}

pub enum RequestParseError {
    InvalidRequest,
    UnsupportedMethod,
    InvalidPath,
    InvalidQueryString,
    InvalidEncoding,
}

impl From<Utf8Error> for RequestParseError {
    fn from(_: Utf8Error) -> Self {
        RequestParseError::InvalidEncoding
    }
}

impl From<MethodError> for RequestParseError {
    fn from(_: MethodError) -> Self {
        RequestParseError::UnsupportedMethod
    }
}


impl RequestParseError{
    fn message(&self) -> &str {
        match self {
            RequestParseError::InvalidRequest => "Invalid HTTP request format",
            RequestParseError::UnsupportedMethod => "Unsupported HTTP method",
            RequestParseError::InvalidPath => "Invalid request path",
            RequestParseError::InvalidQueryString => "Invalid query string format",
            RequestParseError::InvalidEncoding => "Invalid encoding in request",
        }
    }
}

impl Display for RequestParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for RequestParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestParseError: {}", self.message())
    }
}

impl Error for RequestParseError {}