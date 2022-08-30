use std::{error::Error, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
pub struct StpServer {
    listener: TcpListener,
}

impl StpServer {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }
    pub async fn incoming(&self) -> Result<(TcpStream, SocketAddr), std::io::Error> {
        self.listener.accept().await
    }
}