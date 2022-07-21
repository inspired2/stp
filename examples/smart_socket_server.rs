use std::{error::Error, io::Read, net::TcpStream};
use stp::{StpServer, recv_string, send_string};
use smart_house::{PowerSocket, DeviceInfoProvider, SmartDeviceList, Command, CommandData};

fn main() -> Result<(), Box<dyn Error>> {
    let config = stp::get_configuration();
    let server = StpServer::bind(config)?;
    let mut devices = SmartDeviceList::new();

    loop {
        if let Some(mut stream) = server.incoming().next() {
           handle_connection(stream?, &mut devices);
        }
    }

}
fn handle_connection<S: DeviceInfoProvider>(mut stream: TcpStream, devices: &mut S) -> Result<(), Box<dyn Error>> {
    while let Ok(s) = crate::recv_string(&mut stream) {
        let command: Command = serde_json::from_str(&s)?;
        match &command {
            Command::Execute(cmd) => {
                devices.execute_command(cmd)?;
                println!("command executed");

            },
            Command::Unknown => {},
            Command::Exit => {}
        }
        crate::send_string(&mut stream, "exec_result")?;
    }   
    Ok(())
}