use tcp_smart_socket::SmartSocketClient;

#[tokio::main]
async fn main() -> Result<(), ()> { 
    let mut client = SmartSocketClient::with_addr("127.0.0.1:8282").await.unwrap();
    let mut stdin = std::io::stdin();
    let mut buf = String::new();
    loop {
        stdin.read_line(&mut buf).unwrap();
        println!("buf: {:?}", &buf);
        let res = match buf.chars().take(1).collect::<Vec<char>>()[0] {
            '1' => client.turn_on().await,
            '0' => client.turn_off().await,
            '2' => client.get_status().await,
            _ => return Ok(())

        }.unwrap();
        println!("Response: {:?}", res);
        buf.truncate(0);
    }
}