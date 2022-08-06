mod client;
mod config;
mod server;

use crate::config::Settings;
use std::{
    error::Error,
    io::{Read, Write},
};

pub use client::*;
pub use config::*;
pub use server::*;
pub use smart_house::*;

pub fn send_string<D: AsRef<str>, W: Write>(mut dest: W, data: D) -> Result<(), Box<dyn Error>> {
    let data = data.as_ref().as_bytes();
    let len = data.len() as u32;
    let len_bytes = len.to_be_bytes();
    dest.write_all(&len_bytes)?;
    dest.write_all(data)?;
    Ok(())
}
pub fn recv_string<R: Read>(mut dest: R) -> Result<String, Box<dyn Error>> {
    let mut buf = [0_u8; 4];
    dest.read_exact(&mut buf)?;
    let str_len = u32::from_be_bytes(buf);
    let mut str_buf = vec![0_u8; str_len as usize];
    dest.read_exact(&mut str_buf)?;
    let string = String::from_utf8(str_buf)?;
    Ok(string)
}

pub fn get_configuration() -> Result<Settings, Box<dyn std::error::Error>> {
    let local_dir = std::env::current_dir()?;
    let path = local_dir.join("settings.json");
    println!("{:?}", &path);
    if !path.exists() {
        println!("Unable to find settings.json (must be in std::env::current_dir()). Will load default config");
    }
    Ok(Settings::from_file(path).unwrap_or_default())
}
