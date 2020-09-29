extern crate ctrlc;
use server_from_scratch::{http_request::Request, listen_and_serve};
use std::{io, str, process::exit};

const RES_HEADERS: &str = "\
HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=UTF-8\r\n\r\n";

const HELLO_WORLD: &str = "\
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
    ctrlc::set_handler(move || {
        println!("caught!");
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    listen_and_serve(8080, handler)
}

fn handler(req: Request) -> Vec<u8> {
    println!("Req: {:?}", req);

    let res_headers: Vec<u8> = RES_HEADERS.as_bytes().to_owned();
    let res_content: Vec<u8> = HELLO_WORLD.as_bytes().to_owned();

    let mut res = vec![];

    res.extend(res_headers);
    res.extend(res_content);

    res
}
