mod http_server {
    use nix::sys::socket;
    use nix::sys::socket::{
        AddressFamily, InetAddr, IpAddr, MsgFlags, Shutdown, SockAddr, SockProtocol, SockType,
    };
    use std::str;

    const RES_HEADERS: &str = "\
HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
    const HELLO_WORLD: &str = "\
<!DOCTYPE html><html><head><title>hello</title></head><body><h1>HELLO WORLD</h1></body></html>\r\n\r\n";

    pub struct HttpServer {
        server_socket_fd: i32,
    }

    impl HttpServer {
        pub fn new(port: u16) -> Result<Self, nix::Error> {
            let mut http_server = HttpServer {
                server_socket_fd: 0,
            };

            http_server.set_socket_fd()?;
            http_server.bind_socket(port)?;
            http_server.listen_and_serve()?;

            Ok(http_server)
        }

        fn set_socket_fd(&mut self) -> Result<(), nix::Error> {
            self.server_socket_fd = socket::socket(
                AddressFamily::Inet,
                SockType::Stream,
                socket::SockFlag::empty(),
                SockProtocol::Tcp,
            )?;

            Ok(())
        }

        fn bind_socket(&mut self, port: u16) -> Result<(), nix::Error> {
            let ip_addr = IpAddr::new_v4(127, 0, 0, 1);
            let inet_address = InetAddr::new(ip_addr, port);
            let socket_address = &SockAddr::new_inet(inet_address);

            socket::bind(self.server_socket_fd, socket_address)
        }

        fn listen_and_serve(&mut self) -> Result<(), nix::Error> {
            socket::listen(self.server_socket_fd, 1)?;

            loop {
                self.handle_connection()?
            }
        }

        fn handle_connection(&self) -> Result<(), nix::Error> {
            let client_socket_fd = socket::accept(self.server_socket_fd)?;

            let mut buffer = vec![0; 30000];
            socket::recvfrom(client_socket_fd, &mut buffer)?;

            let request_bytes = &buffer.to_owned();
            let request_content = str::from_utf8(request_bytes).unwrap();

            println!("Request content:\n\n{}", request_content);

            let mut res_headers: Vec<u8> = RES_HEADERS.as_bytes().to_owned();
            let res_content: Vec<u8> = HELLO_WORLD.as_bytes().to_owned();
            res_headers.extend(res_content);
            let res = res_headers.as_slice();

            socket::send(client_socket_fd, res, MsgFlags::empty())?;
            socket::shutdown(client_socket_fd, Shutdown::Both)?;

            Ok(())
        }
    }
}

pub fn listen_and_serve(port: u16) -> Result<http_server::HttpServer, nix::Error> {
    http_server::HttpServer::new(port)
}