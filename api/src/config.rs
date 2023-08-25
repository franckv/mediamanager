use std::env::var;
use std::path::Path;

use figment::providers::{Format, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = include_str!("../../config/default.conf");

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub network: NetworkConfig,
    pub library: LibraryConfig,
    pub ripper: RipperConfig,
}

impl Config {
    pub fn new() -> Self {
        let system_config = "/etc/mediamanager.conf";
        let user_config = format!("{}/.config/mediamanager.conf", var("HOME").unwrap());

        if Path::new(&user_config).exists() {
            log::debug!("Load user config");
            Config::load(&user_config)
        } else if Path::new(&system_config).exists() {
            log::debug!("Load system config");
            Config::load(&system_config)
        } else {
            log::debug!("Load default config");
            Config::default()
        }
    }

    pub fn load(config_file: &str) -> Self {
        Figment::new()
            .merge(Toml::string(DEFAULT_CONFIG))
            .merge(Toml::file(config_file))
            .extract()
            .unwrap()
    }
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
pub struct LibraryConfig {
    pub base_dir: String,
    pub create_dir_cmd: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RipperConfig {
    pub dvd: DvdRipperConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DvdRipperConfig {
    pub rip_cmd: String,
    pub label_cmd: String,
}
