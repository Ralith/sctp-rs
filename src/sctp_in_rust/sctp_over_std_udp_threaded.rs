use std::net::UdpSocket;
use super::sctp_connection;
use sockets_api;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::Error;
use std::thread;

pub struct SctpOverUdpThreaded
{
    sctp_conn: sctp_connection::SctpConnection,
    socket:UdpSocket,
}

impl SctpOverUdpThreaded
{
    pub fn new(addr: SocketAddr) -> SctpOverUdpThreaded {
        let socket = UdpSocket::bind(addr).unwrap();

        SctpOverUdpThreaded {
            sctp_conn: sctp_connection::SctpConnection::new(),
            socket: socket,
        }
    }

}

impl sockets_api::SocketsApi for SctpOverUdpThreaded
{
    /// Doesn't actually create a socket, since Rust std's UDP socket needs to be bound at creation.
    fn socket(&self, ipv: sockets_api::IPV) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Binds the socket to an address. You can give either an ipv6 or ipv4 address.
    fn bind(&mut self, addr: SocketAddr) -> Result<(), Error>
    {
        self.socket = UdpSocket::bind(addr)?;

        Ok(())
    }
    /// Starts to listen for connections. Will be unimplemented on the non-native, non-threaded API.
    /// On the threaded UDP API this will start the background thread(s).
    /// On the native implentation, it will map to native listen() call.
    fn listen(&self) -> Result<(), &'static str>
    {
       thread::spawn(move || {
           // TODO
           // Read messages in a loop to detect corrupted/missing messages.
       });

       Ok(())
    }
    /// Accept an incoming connection.
    fn accept(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Connects a client
    fn connect(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Send data over the socket.
    fn send(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Receive data over the socket.
    fn recv(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
    /// Close the socket.
    fn close(&self) -> Result<(), &'static str>
    {
        Ok(())
    }

    fn set_nonblocking(&mut self, block: bool) -> Result<(), Error>
    {
        self.socket.set_nonblocking(block)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let sctp_over_udp_server = SctpOverUdpThreaded::new(SocketAddr::new(IpAddr::V4(<Ipv4Addr>::new(127, 0, 0, 1)), 34254));
        sctp_over_udp_server.listen();
        sctp_over_udp_server.set_nonblocking(true);
        sctp_over_udp_server.accept();

        let sctp_over_udp_client = SctpOverUdpThreaded::new(SocketAddr::new(IpAddr::V4(<Ipv4Addr>::new(0, 0, 0, 0)), 0));
        
    }

    #[test]
    fn create_udp_conn() {
        //let sctp_over_udp = SctpOverUdpThreaded::new();
    }
}