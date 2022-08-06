use smart_house::{Command, ExecutionResult, PowerSocket, SmartDeviceList};
use std::{
    error::Error,
    net::TcpStream,
    sync::{Arc, Mutex},
};
use stp::{recv_string, send_string, StpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let config = stp::get_configuration()?;
    let server = StpServer::bind(config)?;
    let devices = create_device_list();
    let arc_devices = Arc::new(Mutex::new(devices));

    loop {
        if let Some(stream) = server.incoming().next() {
            let stream = stream?;
            println!("Incoming connection: {:?}", stream.peer_addr());
            let cloned_devices = arc_devices.clone();
            std::thread::spawn(move || {
                handle_connection(stream, cloned_devices).ok();
            });
        }
    }
}
fn handle_connection(
    mut stream: TcpStream,
    devices: Arc<Mutex<SmartDeviceList>>,
) -> Result<(), Box<dyn Error>> {
    while let Ok(s) = crate::recv_string(&mut stream) {
        let command: Command = serde_json::from_str(&s)?;
        println!("incoming command: {:?}", command);
        match command {
            Command::Execute(cmd) => {
                let mut devices = devices.lock().unwrap();
                let response = match devices.execute_command(cmd) {
                    Ok(res) => res,
                    Err(e) => ExecutionResult::Error(e),
                };
                let serialized = serde_json::to_string(&response)?;
                crate::send_string(&mut stream, serialized)?;
            }
            Command::Unknown => {
                println!("Received unknown command. Skipping");
                continue;
            }
            Command::Exit => {
                println!("Received exit command");
                break;
            }
        }
    }
    Ok(())
}
fn create_device_list() -> SmartDeviceList {
    let mut list = SmartDeviceList::new();
    let socket1 = PowerSocket {
        name: "s1".into(),
        state: stp::PowerSocketState::NotPowered,
        description: "some desc".into(),
        power_consumption: 0,
    };
    let socket2 = PowerSocket {
        name: "s2".into(),
        state: stp::PowerSocketState::NotPowered,
        description: "some desc".into(),
        power_consumption: 0,
    };
    list.add_device("hall", stp::SmartDevice::Socket(socket1))
        .unwrap();
    list.add_device("bedroom", stp::SmartDevice::Socket(socket2))
        .unwrap();
    list
}
