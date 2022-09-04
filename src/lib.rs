mod client;
mod config;
mod server;

use crate::config::Settings;
pub use client::*;
pub use config::*;
pub use server::*;
pub use smart_house::CustomError;
pub use smart_house::*;

use std::error::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn send_string<D: AsRef<str>, W: AsyncWrite + Unpin>(
    mut dest: W,
    data: D,
) -> Result<(), Box<dyn Error>> {
    println!("enter send_string");
    let data = data.as_ref().as_bytes();
    let len = data.len() as u32;
    let len_bytes = len.to_be_bytes();
    dest.flush().await?;
    println!("begin writint to stream");
    dest.write_all(&len_bytes).await?;
    dest.write_all(data).await?;
    dest.flush().await?;
    println!("before returning");
    Ok(())
}
pub async fn recv_string<R: AsyncRead + Unpin>(mut dest: R) -> Result<String, String> {
    let mut buf = [0_u8; 4];
    dest.read_exact(&mut buf).await.ok();
    let str_len = u32::from_be_bytes(buf);
    let mut str_buf = vec![0_u8; str_len as usize];
    dest.read_exact(&mut str_buf).await.ok();
    let string = String::from_utf8(str_buf).unwrap();
    Ok(string)
}

pub async fn get_configuration() -> Result<Settings, Box<dyn std::error::Error>> {
    let local_dir = std::env::current_dir()?;
    let path = local_dir.join("settings.json");
    if !path.exists() {
        println!("Unable to find settings.json (must be in std::env::current_dir()). Will load default config");
    }
    Ok(Settings::from_file_or_default(path))
}
