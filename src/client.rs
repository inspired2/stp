use smart_house::{DeviceCommand, ExecutionResult, PowerSocketCommand};
use tokio::net::{TcpStream, ToSocketAddrs};
#[derive(Debug)]
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
        println!("waiting for response");
        crate::recv_string(&mut self.stream).await
    }
}
#[derive(Debug)]
pub struct SmartSocketClient {
    connection: StpClient,
}

impl SmartSocketClient {
    pub async fn with_addr(addr: impl ToSocketAddrs) -> Result<Self, String> {
        let connection = StpClient::connect(addr).await.map_err(|e| e.to_string())?;
        Ok(Self { connection })
    }
    async fn send_command(&mut self, data: PowerSocketCommand) -> Result<ExecutionResult, String> {
        let device_command = DeviceCommand::PowerSocket(data);
        let res = self
            .connection
            .send_req(serde_json::to_string(&device_command).unwrap())
            .await
            .map(|r| serde_json::from_str::<ExecutionResult>(&r))?
            .map_err(|e| e.to_string());
        res
    }
    pub async fn turn_on(&mut self) -> Result<ExecutionResult, String> {
        let command = PowerSocketCommand::TurnOn;
        self.send_command(command).await
    }
    pub async fn turn_off(&mut self) -> Result<ExecutionResult, String> {
        let command = PowerSocketCommand::TurnOff;
        self.send_command(command).await
    }
    pub async fn get_status(&mut self) -> Result<ExecutionResult, String> {
        let command = PowerSocketCommand::GetState;
        self.send_command(command).await
    }
}
