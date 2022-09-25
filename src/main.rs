#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use server::Server;
use http::Request;
use http::HttpMethod;
use web_handler::WebHandler;
use std::env;

mod server;
mod http;
mod web_handler;

fn main() {
    let method: HttpMethod = HttpMethod::GET;
    let addr = String::from("127.0.0.1:8080".to_string());
    let server = Server::new(addr);
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public Path: {}", public_path);

    server.run(WebHandler::new(public_path));
}



