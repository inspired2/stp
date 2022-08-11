use std::{
    fs::File,
    net::{Ipv4Addr},
    path::Path,
};

use tokio::net::ToSocketAddrs;
#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    host: String,
    port: u16,
}

impl Settings {
    pub fn from_file_or_default<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(file) = File::open(path) {
            let settings: Settings = serde_json::from_reader(file).unwrap_or_default();
            if settings.is_valid() {
                return settings
            }
        }
        Self::default()
    }
    pub fn get_addr(&self) -> impl ToSocketAddrs {
        //safe to unwrap as already checked is_valid
        let v4: Ipv4Addr = self.host.to_owned().parse().unwrap();
        (v4, self.port)
    }
    pub fn is_valid(&self) -> bool {
        if self.host.parse::<Ipv4Addr>().is_err() { return false }
        true
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
        }
    }
}