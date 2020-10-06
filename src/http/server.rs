use super::{request::Request, socket::Socket, thread_pool::ThreadPool};
use std::{
    io,
    str::{self, FromStr},
};

pub trait Handler: Clone + Send + Sync + 'static {
    fn serve_http(&self, req: Request) -> io::Result<Vec<u8>>;
}

impl<F> Handler for F
where
    F: Fn(Request) -> io::Result<Vec<u8>> + Clone + Send + Sync + 'static,
{
    fn serve_http(&self, req: Request) -> io::Result<Vec<u8>> {
        self(req)
    }
}

pub struct HttpServer {
    port: u16,
    socket: Socket,
    pool: ThreadPool,
}

impl HttpServer {
    pub fn new(port: u16) -> io::Result<Self> {
        let socket = Socket::new()?;
        socket.bind(port)?;

        let pool = ThreadPool::new(4);

        Ok(Self { port, socket, pool })
    }

    pub fn listen_and_serve(&self, handler: impl Handler) -> io::Result<()> {
        self.socket.listen(128)?;
        println!("Server started on port: {}", self.port);

        for client_socket in self.socket.incoming() {
            let handler = handler.clone();

            self.pool.execute(move || -> io::Result<()> {
                let read_buffer = &mut [0; 30000];
                client_socket.receive(read_buffer).unwrap();

                let req_str = str::from_utf8(read_buffer).unwrap();
                let req = Request::from_str(req_str).unwrap();

                let res = handler.serve_http(req)?;
                client_socket.send(&res).unwrap();

                // if req.url.path == "/" {
                //     let res = handler.serve_http(req)?;
                //     client_socket.send(&res).unwrap();
                // } else {
                //     client_socket
                //         .send("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())
                //         .unwrap();
                // }

                Ok(())
            });
        }

        Ok(())
    }
}
