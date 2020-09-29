pub mod http_request;
pub mod http_server;
pub mod common;
pub mod socket;

use std::io;
use http_request::Request;

pub fn listen_and_serve<F>(port: u16, handler: F) -> io::Result<()>
where
    F: Fn(Request) -> Vec<u8>,
{
    http_server::HttpServer::new(port)?.listen_and_serve(handler)
}
