use super::{http_request::Request, socket::Socket};
use std::{
    io,
    str::{self, FromStr},
};

pub struct HttpServer {
    pub socket: Socket,
}

impl HttpServer {
    pub fn new(port: u16) -> io::Result<Self> {
        let socket = Socket::new()?;

        socket.bind(port)?;

        Ok(HttpServer { socket })
    }

    pub fn listen_and_serve<F>(&self, handler: F) -> io::Result<()>
    where
        F: Fn(Request) -> Vec<u8>,
    {
        self.socket.listen(128)?;

        for client_socket in self.socket.incoming() {
            let mut read_buffer = vec![0; 30000];

            client_socket.receive(&mut read_buffer)?;

            let req_string = str::from_utf8(&read_buffer)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "couldn't read request"))?;

            let req = Request::from_str(req_string).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "couldn't create request structure",
                )
            })?;

            let res = handler(req);

            client_socket.send(&res)?;
        }

        Ok(())
    }
}
