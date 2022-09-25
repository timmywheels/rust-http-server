use std::str::FromStr;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
}

impl FromStr for HttpMethod {
    type Err = HttpMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(HttpMethodError),
        }
    }
}

pub struct HttpMethodError;