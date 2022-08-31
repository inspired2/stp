use tcp_smart_socket::SocketServer;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let addr = "127.0.0.1:8282";
    let smart_socket = smart_house::PowerSocket { name: "some_socket".into(), state: smart_house::PowerSocketState::NotPowered, description: "socket in a room".into(), power_consumption: 0 };
    
    let mut server = SocketServer::with_addr(addr, smart_socket).await.unwrap();
    let handle = tokio::spawn(async move {server.run().await});
    let stdin = std::io::stdin();
    let mut buf = String::new();
    println!("Type 'exit' to close server");
    while let Ok(_) = stdin.read_line(&mut buf) {
        if buf.as_str() == "exit\n" { break }
        buf.truncate(0);
    }   
    handle.abort();
    Ok(())
}