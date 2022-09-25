use super::http::{Request, Response, HttpStatusCode, HttpMethod};
use super::server::Handler;
use std::fs;

pub struct WebHandler {
    public_path: String,
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            HttpMethod::GET => match request.path() {
                "/" => Response::new(HttpStatusCode::Ok, self.read_file("index.html")),
                "/hello-world" => Response::new(HttpStatusCode::Ok, self.read_file("hello-world.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(HttpStatusCode::Ok, Some(contents)),
                    None => Response::new(HttpStatusCode::NotFound, None)
                }
            }
            _ => Response::new(HttpStatusCode::NotFound, None)
        }
    }
}