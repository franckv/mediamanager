use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub network: NetworkConfig,
    pub ripper: RipperConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    pub address: [u8; 4],
    pub port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            address: [0, 0, 0, 0],
            port: 3000,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RipperConfig {
    pub dvd: DvdRipperConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DvdRipperConfig {
    pub base_dir: String,
    pub create_dir_cmd: String,
    pub rip_cmd: String,
    pub label_cmd: String,
}
