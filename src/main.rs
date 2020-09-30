use server_from_scratch::net::{http_request::Request, http_server::HttpServer};
use std::{io, str};

const RES: &str = "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=UTF-8\r\n\r\n\
<!DOCTYPE html>\
<html>\
    <head>\
        <title>hello</title>\
    </head>\
    <body>\
        <h1>HELLO WORLD</h1>\
    </body>\
</html>\r\n\r\n";

fn main() -> io::Result<()> {
    let mut server = HttpServer::new(8081)?;
    server.listen_and_serve(handler)
}

fn handler(req: Request) -> Vec<u8> {
    println!("Req: {:?}", req);
    RES.as_bytes().to_owned()
}
