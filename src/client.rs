use tokio::net::{TcpStream, ToSocketAddrs};
pub struct StpClient {
    stream: TcpStream,
}
impl StpClient {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, String> {
        let stream = TcpStream::connect(addr).await.map_err(|e| e.to_string())?;
        Ok(Self { stream })
    }
    pub async fn send_req<R: AsRef<str>>(&mut self, data: R) -> Result<String, String> {
        crate::send_string(&mut self.stream, data)
            .await
            .map_err(|e| e.to_string())?;
        crate::recv_string(&mut self.stream).await
    }
}
