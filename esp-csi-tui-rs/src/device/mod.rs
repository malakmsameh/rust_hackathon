pub mod serial_handler;
pub mod esp_client;

pub use serial_handler::SerialHandler;
pub use esp_client::EspClient;

use crate::models::{CsiMeasurement, DeviceConfig};
use anyhow::Result;

#[async_trait::async_trait]
pub trait Device {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn configure(&mut self, config: DeviceConfig) -> Result<()>;
    async fn start_collection(&mut self) -> Result<()>;
    async fn stop_collection(&mut self) -> Result<()>;
    async fn read_measurement(&mut self) -> Result<Option<CsiMeasurement>>;
}
