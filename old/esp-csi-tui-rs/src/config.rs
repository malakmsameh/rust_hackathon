use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub serial: SerialConfig,
    pub collection: CollectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub port: String,
    pub baud_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub default_channel: u8,
    pub default_duration: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            serial: SerialConfig {
                port: "/dev/ttyUSB0".to_string(),
                baud_rate: 115200,
            },
            collection: CollectionConfig {
                default_channel: 6,
                default_duration: 60,
            },
        }
    }
}

/// Get config file path: ~/.esp-csi/config.toml
pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".esp-csi");
    path.push("config.toml");
    path
}

/// Load configuration from file or use defaults
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    
    // Create parent directory if needed
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(config)?;
    std::fs::write(&config_path, content)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.serial.port, "/dev/ttyUSB0");
        assert_eq!(config.serial.baud_rate, 115200);
        assert_eq!(config.collection.default_channel, 6);
    }

    #[test]
    fn test_serialize_config() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("/dev/ttyUSB0"));
    }
}
