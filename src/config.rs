use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Error;
use serde::Deserialize;
use serde::Serialize;

use crate::utils;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub log_level: String,
    pub server: Server,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub workers: usize,
    pub bind_address: String,
    pub port: u16,
    pub keep_alive: u64,
}

impl Config {
    pub fn config_path() -> PathBuf {
        utils::enforced_data_root().join("config.json")
    }

    pub fn load() -> Result<Self, Error> {
        let config_path: PathBuf = Config::config_path();
        if !config_path.exists() {
            println!(
                "Config not found. Creating a default configuration at {:?}",
                config_path
            );
            let config = Config::default();
            config.save().context("saving default config")?;
            return Ok(config);
        }

        let file_content = fs::read_to_string(&config_path)
            .with_context(|| format!("Could not open config file at {:?}", &config_path))?;
        let config: Config =
            ::serde_json::from_str(file_content.as_str()).context("Could not parse config")?;

        Ok(config)
    }

    /// Save the this config object to a file in toml format
    pub fn save(&self) -> Result<(), Error> {
        let config_path = Config::config_path();
        let content = ::serde_json::to_string_pretty(self).context("Encoding config file")?;
        fs::write(config_path, content).context("Writing config file")?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            log_level: "DEBUG".to_string(),
            server: Server {
                workers: 4,
                bind_address: "0.0.0.0".to_string(),
                port: 8000,
                keep_alive: 10,
            },
            debug: true, // Change this to false when we do a release.
        }
    }
}
