use std::io;
pub mod socket;

mod http_server {
    use super::socket::Socket;
    use std::io;
    use std::str;

    const RES_HEADERS: &str = "\
HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
    const HELLO_WORLD: &str = "\
<!DOCTYPE html><html><head><title>hello</title></head><body><h1>HELLO WORLD</h1></body></html>\r\n\r\n";

    pub struct HttpServer {
        socket: Socket,
    }

    impl HttpServer {
        pub fn new(port: u16) -> io::Result<Self> {
            let socket = Socket::new()?;
            socket.bind(port)?;

            let http_server = HttpServer { socket };
            http_server.listen_and_serve()?;

            Ok(http_server)
        }

        fn listen_and_serve(&self) -> io::Result<()> {
            self.socket.listen(128)?;

            loop {
                self.handle_connection()?;
            }
        }

        fn handle_connection(&self) -> io::Result<()> {
            let client_socket = self.socket.accept()?;

            let mut buffer = vec![0; 30000];
            client_socket.receive(&mut buffer)?;

            let request_bytes = &buffer.to_owned();
            let request_content = str::from_utf8(request_bytes).unwrap();

            println!("Request content:\n\n{}", request_content);

            let mut res_headers: Vec<u8> = RES_HEADERS.as_bytes().to_owned();
            let res_content: Vec<u8> = HELLO_WORLD.as_bytes().to_owned();
            res_headers.extend(res_content);
            let res = res_headers.as_slice();

            client_socket.send(res)?;
            client_socket.shutdown()?;

            Ok(())
        }
    }
}

pub fn listen_and_serve(port: u16) -> io::Result<http_server::HttpServer> {
    http_server::HttpServer::new(port)
}
