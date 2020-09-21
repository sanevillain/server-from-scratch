use nix::sys::socket::{
    accept, bind, listen, recvfrom, send, shutdown, socket, AddressFamily, InetAddr, IpAddr,
    MsgFlags, Shutdown, SockAddr, SockFlag, SockProtocol, SockType,
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

        Ok(Socket { fd })
    }

    pub fn bind(&self, port: u16) -> io::Result<()> {
        let ip_addr = IpAddr::new_v4(127, 0, 0, 1);
        let inet_addr = InetAddr::new(ip_addr, port);
        let socket_addr = &SockAddr::new_inet(inet_addr);

        bind(self.fd, socket_addr).map_err(|err| nix_to_io_error(err, "Socket Bind Error!"))?;
        Ok(())
    }

    pub fn listen(&self, backlog: usize) -> io::Result<()> {
        listen(self.fd, backlog).map_err(|err| nix_to_io_error(err, "Socket Listen Error!"))?;
        Ok(())
    }

    pub fn accept(&self) -> io::Result<Self> {
        let fd = accept(self.fd).map_err(|err| nix_to_io_error(err, "Socket Accept Error!"))?;

        Ok(Socket { fd })
    }

    pub fn receive(&self, buf: &mut [u8]) -> io::Result<usize> {
        let (read_bytes, _) =
            recvfrom(self.fd, buf).map_err(|err| nix_to_io_error(err, "Socket Receive Error!"))?;
        Ok(read_bytes)
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        let sent_bytes = send(self.fd, buf, MsgFlags::empty())
            .map_err(|err| nix_to_io_error(err, "Socket Send Error!"))?;
        Ok(sent_bytes)
    }

    pub fn shutdown(&self) -> io::Result<()> {
        shutdown(self.fd, Shutdown::Both)
            .map_err(|err| nix_to_io_error(err, "Socket Shutdown Error!"))?;
        Ok(())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        self.shutdown().unwrap();
    }
}

fn nix_to_io_error(err: nix::Error, err_message: &'static str) -> io::Error {
    match err.as_errno() {
        Some(err_num) => io::Error::from_raw_os_error(err_num as i32),
        None => io::Error::new(io::ErrorKind::Other, err_message),
    }
}
