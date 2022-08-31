use tcp_smart_socket::SocketServer;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let addr = "127.0.0.1:8082";
    let smart_socket = smart_house::PowerSocket { name: "some_socket".into(), state: smart_house::PowerSocketState::NotPowered, description: "socket in a room".into(), power_consumption: 0 };
    
    let mut server = SocketServer::with_addr(addr, smart_socket).await.unwrap();
    server.run().await;
    
    Ok(())
}