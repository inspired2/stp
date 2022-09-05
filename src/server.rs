use log::{error, info};
use smart_house::{CustomError, DeviceCommand, Executable, ExecutionResult, PowerSocket};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Mutex,
    task::JoinHandle,
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
    handles: Vec<JoinHandle<Result<(), String>>>,
}

impl Drop for SocketServer {
    fn drop(&mut self) {
        self.handles.iter_mut().for_each(|handle| handle.abort());
    }
}

impl SocketServer {
    pub async fn with_addr(addr: impl ToSocketAddrs, socket: PowerSocket) -> Result<Self, String> {
        let server = StpServer::bind(addr).await?;
        info!(
            "Start listening for incoming connections on: {:?}",
            server.listener.local_addr()
        );
        Ok(Self {
            connection: server,
            smart_socket: Arc::new(Mutex::new(socket)),
            handles: Default::default(),
        })
    }
    pub async fn run(&mut self) {
        loop {
            if let Ok((stream, addr)) = self.connection.incoming().await {
                info!("Incoming connection from {:?}", addr);
                let smart_socket = Arc::clone(&self.smart_socket);
                let handle =
                    tokio::spawn(async move { handle_connection(stream, smart_socket).await });
                self.handles.push(handle);
            }
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    device: Arc<Mutex<PowerSocket>>,
) -> Result<(), String> {
    while let Ok(string) = crate::recv_string(&mut stream).await {
        info!("Received command data: {}", &string);
        if let Ok(cmd) = serde_json::from_str::<DeviceCommand>(&string) {
            let device = Arc::clone(&device);

            let mut device = device.lock().await;
            let result = device.execute(cmd);
            info!(
                "Command parsing Ok. Execution Result: {:?}. Sending response",
                &result
            );
            drop(device);

            crate::send_string(&mut stream, serde_json::to_string(&result).unwrap())
                .await
                .map_err(|e| {
                    error!(
                        "Error sending execution result to client at {:?}",
                        stream.peer_addr().map_err(|e| {
                            error!("Failed to get peer_addr of {:?}", &stream);
                            e.to_string()
                        })
                    );
                    e.to_string()
                })?;
        } else {
            error!(
                "Failed to parse DeviceCommand from received command data: {}",
                &string
            );
            let result = ExecutionResult::Error(CustomError::CommandExecutionFailure(
                "Command unknown".into(),
            ));
            crate::send_string(&mut stream, serde_json::to_string(&result).unwrap())
                .await
                .map_err(|e| {
                    error!(
                        "Error sending stringified result to {:?}",
                        stream.peer_addr().map_err(|e| {
                            error!("Failed to get peer_addr of {:?}", &stream);
                            e.to_string()
                        })
                    );
                    e.to_string()
                })?;
        }
    }
    Ok(())
}
