use std::str;
use std::io;

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
    server_from_scratch::listen_and_serve(8080, handler)
}

fn handler(req: &[u8]) -> Vec<u8> {
    let request_bytes = &req.to_owned();
    let request_content = str::from_utf8(request_bytes).unwrap();

    println!("Request content:\n\n{}", request_content);

    let res_headers: Vec<u8> = RES_HEADERS.as_bytes().to_owned();
    let res_content: Vec<u8> = HELLO_WORLD.as_bytes().to_owned();

    let mut res = vec![];

    res.extend(res_headers);
    res.extend(res_content);

    res
}