use std::{
    error::Error,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};


pub struct StpServer {
    listener: TcpListener,
}

impl StpServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }
    pub fn incoming(&self) -> impl Iterator<Item = Result<TcpStream, std::io::Error>> +'_{
        self.listener.incoming()
    }

}
