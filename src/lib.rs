use std::io;
pub mod socket;

mod http_server {
    use super::socket::Socket;
    use std::io;
    use std::collections::HashMap;

    pub struct HttpServer {
        socket: Socket,
    }

    impl HttpServer {
        pub fn new(port: u16) -> io::Result<Self> {
            let socket = Socket::new()?;

            socket.bind(port)?;

            Ok(HttpServer { socket })
        }

        pub fn listen_and_serve<F>(&self, handler: F) -> io::Result<()>
        where
            F: Fn(&[u8]) -> Vec<u8>,
        {
            self.socket.listen(128)?;

            for client_socket in self.socket.incoming() {
                let mut read_buffer = vec![0; 30000];

                client_socket.receive(&mut read_buffer)?;

                let res = handler(&read_buffer);

                client_socket.send(&res)?;
            }

            Ok(())
        }
    }

    pub struct Request {
        headers: HashMap<String, Vec<String>>
    }
}

pub fn listen_and_serve<F>(port: u16, handler: F) -> io::Result<()>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    http_server::HttpServer::new(port)?.listen_and_serve(handler)
}
