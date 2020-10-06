use server_from_scratch::http::{request::Request, server::HttpServer, file_server};
use std::{fs, io, io::prelude::*, path::Path, str};

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
    let server = HttpServer::new(8080)?;

    let handler = file_server::FileServer::new("./src");

    server.listen_and_serve(handler)?;



    Ok(())
}

// fn handler(req: Request) -> io::Result<Vec<u8>> {
//     println!("Req: {:?}", req);
//     Ok(RES.as_bytes().to_owned())
// }
