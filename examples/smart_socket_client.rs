use std::error::Error;
use std::io::Write;
use stp::StpClient;
use stp::{Command, CommandData};

fn main() -> Result<(), Box<dyn Error>> {
    let config = stp::get_configuration();
    let mut client = StpClient::connect(config)?;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    std::thread::spawn(move || loop {
        
        stdin.read_line(&mut buf).ok();
        let command = Command::from(buf.to_owned());
        match &command {
            Command::Execute(CommandData { .. }) => {
                let resp = client
                    .send_req(serde_json::to_string(&command).unwrap())
                    .unwrap();
                writeln!(stdout, "{}", resp).unwrap();
            }
            Command::Exit => break,
            Command::Unknown => {}
        }
        buf.truncate(0);
    })
    .join()
    .unwrap();
    Ok(())
}
