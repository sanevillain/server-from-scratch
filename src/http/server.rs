use super::{request::Request, response::Response, thread_pool::ThreadPool};
use crate::net::socket::Socket;
use std::{io, str::FromStr, time};

pub trait Handler: Clone + Send + Sync + 'static {
    fn serve_http(&self, req: Request) -> io::Result<Response>;
}

impl<F> Handler for F
where
    F: Fn(Request) -> io::Result<Response> + Clone + Send + Sync + 'static,
{
    fn serve_http(&self, req: Request) -> io::Result<Response> {
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
        info!("Server started on port: {}", self.port);

        for client_socket in self.socket.incoming() {
            info!("Got a new request");
            let handler = handler.clone();

            self.pool
                .execute(move || HttpServer::handle_connection(client_socket, handler));
        }

        info!("Shutting down server on port: {}", self.port);
        Ok(())
    }

    fn handle_connection(client_socket: Socket, handler: impl Handler) -> io::Result<()> {
        let now = time::Instant::now();

        let read_buffer = &mut [0; 30000];
        client_socket
            .receive(read_buffer)
            .expect("Clinet Socket receive");

        let req =
            Request::from_str(&String::from_utf8_lossy(read_buffer)).expect("Request build error");
        let res = handler.serve_http(req)?;

        client_socket
            .send(&res.to_bytes())
            .expect("Request send error");

        info!("Finished request in {}", now.elapsed().as_millis());
        Ok(())
    }
}
