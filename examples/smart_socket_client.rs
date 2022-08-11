use std::error::Error;
use std::io::Write;

use tcp_smart_socket::{Command, CommandData};
use tcp_smart_socket::{ExecutionResult, StpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = tcp_smart_socket::get_configuration().await?.get_addr();
    let mut client = StpClient::connect(addr).await?;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();
    loop {
        writeln!(stdout, "Enter device name or 'exit' to quit")?;

        stdin.read_line(&mut buf).ok();
        let device_name = buf.trim().to_owned();
        if device_name.to_lowercase() == "exit" { break }
        buf.truncate(0);

        writeln!(stdout, "Enter command")?;
        writeln!(stdout, "For smart socket:")?;
        writeln!(stdout, "11 - turn on")?;
        writeln!(stdout, "10 - turn off")?;
        writeln!(stdout, "12 - get state")?;

        stdin.read_line(&mut buf).ok();
        let cmd_code = buf.trim().parse::<u8>().unwrap_or(99);
        let command = Command::from((device_name, cmd_code));
        match &command {
            Command::Execute(CommandData { .. }) => {
                let stringified_command = serde_json::to_string(&command).unwrap();
                let resp = client.send_req(stringified_command).await?;
                let exec_res: ExecutionResult = serde_json::from_str(&resp)?;
                writeln!(stdout, "{:?}", exec_res).unwrap();
            }
            Command::Unknown => {
                println!("Unknown command, please enter valid data");
            }
        }
        buf.truncate(0);
    }
    Ok(())
}
