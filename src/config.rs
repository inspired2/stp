use serde_json;
use std::{
    error::Error,
    fs::File,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, AddrParseError},
    path::Path, io::ErrorKind,
};

#[derive(serde::Deserialize)]
pub struct Settings {
    host: String,
    port: u16,
}

impl Settings {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let settings: Settings = serde_json::from_reader(file)?;
        Ok(settings)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 8080,
        }
    }
}
impl ToSocketAddrs for Settings {
    type Iter = Box<dyn Iterator<Item = SocketAddr>>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        let ip: Ipv4Addr = match &self.host {
            s if s == "localhost" => Ipv4Addr::new(127, 0, 0, 1),
            s => s.to_owned().parse().map_err(|_| ErrorKind::InvalidData)?,
        };
        let addr = SocketAddr::new(IpAddr::V4(ip), self.port);
        Ok(Box::new(std::iter::once(addr)))
    }
}
