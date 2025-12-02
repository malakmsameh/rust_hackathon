pub mod rerun_client;

use crate::models::CsiMeasurement;
use anyhow::Result;

#[async_trait::async_trait]
pub trait StreamingProvider {
    async fn connect(&mut self) -> Result<()>;
    async fn send_measurement(&self, measurement: &CsiMeasurement) -> Result<()>;
    async fn close(&mut self) -> Result<()>;
}
