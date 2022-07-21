use std::{net::{TcpStream, ToSocketAddrs}, error::Error};

pub struct StpClient {
    stream: TcpStream
}
impl StpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            stream
        })
    }
    pub fn send_req<R: AsRef<str>>(&mut self, data: R) -> Result<String, Box<dyn Error>> {
        crate::send_string(&mut self.stream, data)?;
        crate::recv_string(&mut self.stream)
    }
}