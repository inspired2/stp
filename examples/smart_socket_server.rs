use smart_house::{Command, PowerSocket, SmartDeviceList};
use std::{error::Error, sync::Arc};
use tcp_smart_socket::{recv_string, send_string, StpServer};
use tokio::net::TcpStream;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = tcp_smart_socket::get_configuration().await?.get_addr();

    let server = StpServer::bind(addr).await?;
    let devices = create_device_list();
    let arc_devices = Arc::new(devices);

    loop {
        if let Ok((stream, addr)) = server.incoming().await {
            println!("Incoming connection: {:?}", addr);
            let cloned_devices = arc_devices.clone();
            tokio::spawn(async {
                handle_connection(stream, cloned_devices).await.ok();
            });
        }
    }
}
async fn handle_connection(
    mut stream: TcpStream,
    devices: Arc<SmartDeviceList>,
) -> Result<(), String> {
    while let Ok(s) = crate::recv_string(&mut stream).await {
        if serde_json::from_str::<Command>(&s).is_err() {
            continue;
        }
        let command: Command = serde_json::from_str(&s).unwrap();
        println!("incoming command: {:?}", command);
        match command {
            Command::Execute(cmd) => {
                let response = devices.execute_command(cmd);
                let serialized = serde_json::to_string(&response).map_err(|e| e.to_string())?;
                crate::send_string(&mut stream, serialized)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            Command::Unknown => {
                println!("Received unknown command. Skipping");
                continue;
            }
        }
    }
    Ok(())
}
fn create_device_list() -> SmartDeviceList {
    let mut list = SmartDeviceList::new();
    let socket1 = PowerSocket {
        name: "s1".into(),
        state: tcp_smart_socket::PowerSocketState::NotPowered,
        description: "some desc".into(),
        power_consumption: 0,
    };
    let socket2 = PowerSocket {
        name: "s2".into(),
        state: tcp_smart_socket::PowerSocketState::NotPowered,
        description: "some desc".into(),
        power_consumption: 0,
    };
    list.add_device("hall", tcp_smart_socket::SmartDevice::Socket(socket1))
        .unwrap();
    list.add_device("bedroom", tcp_smart_socket::SmartDevice::Socket(socket2))
        .unwrap();
    list
}
