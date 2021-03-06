use nix::{
    sys::socket::{
        accept, bind, listen, recvfrom, send, setsockopt, socket, sockopt::ReuseAddr,
        AddressFamily, InetAddr, IpAddr, MsgFlags, SockAddr, SockFlag, SockProtocol, SockType,
    },
    unistd::close,
};

use std::io;

pub struct Socket {
    fd: i32,
}

impl Socket {
    pub fn new() -> io::Result<Self> {
        let fd = socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            SockProtocol::Tcp,
        )
        .map_err(|err| nix_to_io_error(err, "Socket Allocation Error!"))?;

        setsockopt(fd, ReuseAddr, &true)
            .map_err(|err| nix_to_io_error(err, "Socket Allocation Options Error!"))?;
        debug!("New Socket {} created.", fd);
        Ok(Socket { fd })
    }

    pub fn bind(&self, port: u16) -> io::Result<()> {
        let ip_addr = IpAddr::new_v4(127, 0, 0, 1);
        let inet_addr = InetAddr::new(ip_addr, port);
        let socket_addr = &SockAddr::new_inet(inet_addr);

        bind(self.fd, socket_addr).map_err(|err| nix_to_io_error(err, "Socket Bind Error!"))?;
        debug!("Socket {} bound.", self.fd);
        Ok(())
    }

    pub fn listen(&self, backlog: usize) -> io::Result<()> {
        listen(self.fd, backlog).map_err(|err| nix_to_io_error(err, "Socket Listen Error!"))?;
        debug!("Socket {} listening.", self.fd);
        Ok(())
    }

    pub fn accept(&self) -> io::Result<Self> {
        let fd = accept(self.fd).map_err(|err| nix_to_io_error(err, "Socket Accept Error!"))?;
        debug!("Socket {} accepted new Socket {}.", self.fd, fd);
        Ok(Socket { fd })
    }

    pub fn receive(&self, buf: &mut [u8]) -> io::Result<usize> {
        let (read_bytes, _) =
            recvfrom(self.fd, buf).map_err(|err| nix_to_io_error(err, "Socket Receive Error!"))?;
        debug!("Reading from socket {}.", self.fd);
        Ok(read_bytes)
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        let sent_bytes = send(self.fd, buf, MsgFlags::empty())
            .map_err(|err| nix_to_io_error(err, "Socket Send Error!"))?;
        debug!("Sending to socket {}.", self.fd);
        Ok(sent_bytes)
    }

    pub fn shutdown(&self) -> io::Result<()> {
        close(self.fd).map_err(|err| nix_to_io_error(err, "Socket Shutdown Error!"))?;
        Ok(())
    }

    pub fn incoming(&self) -> Connections {
        Connections::new(self)
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        match self.shutdown() {
            Ok(_) => debug!("Socket {} closed.", self.fd),
            Err(e) => error!("Coudln't close socket {}. {:?}", self.fd, e),
        }
    }
}

pub struct Connections<'a> {
    listener: &'a Socket,
}

impl<'a> Connections<'a> {
    pub fn new(listener: &'a Socket) -> Self {
        Self { listener }
    }
}

impl<'a> Iterator for Connections<'a> {
    type Item = Socket;

    fn next(&mut self) -> Option<Self::Item> {
        self.listener.accept().ok()
    }
}

fn nix_to_io_error(err: nix::Error, err_message: &'static str) -> io::Error {
    match err.as_errno() {
        Some(err_num) => io::Error::from_raw_os_error(err_num as i32),
        None => io::Error::new(io::ErrorKind::Other, err_message),
    }
}
