use server_from_scratch::http::{file_server, server::HttpServer};
use std::io;

fn main() -> io::Result<()> {
    let server = HttpServer::new(8080)?;
    let handler = file_server::FileServer::new("/Users/doom/go");
    server.listen_and_serve(handler)?;
    Ok(())
}
