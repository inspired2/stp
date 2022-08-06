use std::error::Error;
use std::io::Write;

use stp::{Command, CommandData};
use stp::{ExecutionResult, StpClient};

fn main() -> Result<(), Box<dyn Error>> {
    let config = stp::get_configuration()?;
    let mut client = StpClient::connect(config)?;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();
    loop {
        writeln!(stdout, "Enter device name")?;

        stdin.read_line(&mut buf).ok();
        let device_name = buf.trim().to_owned();
        buf.truncate(0);

        writeln!(stdout, "Enter command")?;
        writeln!(stdout, "For smart socket:")?;
        writeln!(stdout, "11 - turn on")?;
        writeln!(stdout, "10 - trun off")?;

        stdin.read_line(&mut buf).ok();
        let cmd_code = buf.trim().parse::<u8>().unwrap_or(99);
        let command = Command::from((device_name, cmd_code));
        match &command {
            Command::Execute(CommandData { .. }) => {
                let stringified_command = serde_json::to_string(&command).unwrap();
                let resp = client.send_req(stringified_command)?;
                let exec_res: ExecutionResult = serde_json::from_str(&resp)?;
                writeln!(stdout, "{:?}", exec_res).unwrap();
            }
            Command::Exit => break,
            Command::Unknown => {
                println!("Unknown command, please enter valid data");
            }
        }
        buf.truncate(0);
    }
    Ok(())
}
