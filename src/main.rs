extern crate ctrlc;
use server_from_scratch::{http_request::Request, listen_and_serve};
use std::{io, process::exit, str};

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
    // ctrlc::set_handler(move || {
    //     println!("caught!");
    //     exit(0);
    // }).expect("Error setting Ctrl-C handler");

    listen_and_serve(8080, handler)
}

fn handler(req: Request) -> Vec<u8> {
    println!("Req: {:?}", req);
    RES.as_bytes().to_owned()
}
