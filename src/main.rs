use server_from_scratch::http::{file_server, server::HttpServer};
use std::io;

fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let server = HttpServer::new(8080)?;
    let handler = file_server::FileServer::new("/Users/doom/Downloads");
    server.listen_and_serve(handler)?;

    Ok(())
}
