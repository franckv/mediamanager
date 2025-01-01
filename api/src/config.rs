use std::env::var;
use std::path::Path;

use figment::providers::{Format, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = include_str!("../../config/default.conf");

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
            log::info!("Load user config");
            Config::load(&user_config)
        } else if Path::new(&system_config).exists() {
            log::info!("Load system config");
            Config::load(system_config)
        } else {
            log::info!("Load default config");
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LibraryConfig {
    pub base_dir: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RipperConfig {
    pub eject: bool,
    pub mock: bool,
    pub create_dir_cmd: String,
    pub dvd: DvdRipperConfig,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DvdRipperConfig {
    pub rip_cmd: String,
    pub label_cmd: String,
}
