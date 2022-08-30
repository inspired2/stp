use smart_house::{DeviceCommand, Executable, PowerSocket};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Mutex,
};
pub struct StpServer {
    listener: TcpListener,
}

impl StpServer {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, String> {
        let listener = TcpListener::bind(addr).await.map_err(|e| e.to_string())?;
        Ok(Self { listener })
    }
    pub async fn incoming(&self) -> Result<(TcpStream, SocketAddr), std::io::Error> {
        self.listener.accept().await
    }
}

pub struct SocketServer {
    smart_socket: Arc<Mutex<PowerSocket>>,
    connection: StpServer,
}
impl SocketServer {
    pub async fn with_addr(addr: impl ToSocketAddrs, socket: PowerSocket) -> Result<Self, String> {
        let server = StpServer::bind(addr).await?;
        Ok(Self {
            connection: server,
            smart_socket: Arc::new(Mutex::new(socket)),
        })
    }
    pub async fn run(&mut self) {
        loop {
            if let Ok((stream, _addr)) = self.connection.incoming().await {
                let smart_socket = Arc::clone(&self.smart_socket);
                tokio::spawn(async move { handle_connection(stream, smart_socket).await });
            }
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    device: Arc<Mutex<PowerSocket>>,
) -> Result<(), String> {
    while let Ok(string) = crate::recv_string(&mut stream).await {
        if let Ok(cmd) = serde_json::from_str::<DeviceCommand>(&string) {
            let device = Arc::clone(&device);
            let mut device = device.lock().await;
            let result = device.execute(cmd);
            drop(device);
            crate::send_string(&mut stream, serde_json::to_string(&result).unwrap()).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
