use serde_json::json;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpStream, ToSocketAddrs};
pub struct StpClient {
    stream: TcpStream,
}
impl StpClient {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, String> {
        let stream = TcpStream::connect(addr).await.map_err(|e| e.to_string())?;
        Ok(Self { stream })
    }
    pub async fn send_req<R: AsRef<str>>(&mut self, data: R) -> Result<String, String> {
        crate::send_string(&mut self.stream, data)
            .await
            .map_err(|e| e.to_string())?;
        crate::recv_string(&mut self.stream).await
    }
}

pub struct SmartSocketClient {
    connection: StpClient
}

impl SmartSocketClient {
    pub async fn with_addr(addr: impl ToSocketAddrs) -> Result<Self, String> {
        let connection = StpClient::connect(addr).await.map_err(|e| e.to_string())?;
        Ok(Self {connection})
    }
    async fn send_command(&mut self, data: SmartSocketCommand) -> Result<SmartSocketResponse, String> {
        let res = self.connection
            .send_req(json!(data).to_string())
            .await
            .map(|r| serde_json::from_str::<SmartSocketResponse>(&r))?
            .map_err(|e| e.to_string());
        res

    }
    pub async fn turn_on(&mut self) -> Result<SmartSocketResponse, String> {
        let command = SmartSocketCommand::TurnOn;
        self.send_command(command).await

    }
    pub async fn turn_off(&mut self) -> Result<SmartSocketResponse, String> {
        let command = SmartSocketCommand::TurnOff;
        self.send_command(command).await
    }
    pub async fn get_status(&mut self) -> Result<SmartSocketResponse, String> {
        let command = SmartSocketCommand::Status;
        self.send_command(command).await
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SmartSocketResponse {
    Ok(String),
    Err(String)
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SmartSocketCommand {
    TurnOn,
    TurnOff,
    Status
}