use crate::models::StreamingConfig;
use anyhow::Result;

pub struct RerunClient {
    config: StreamingConfig,
}

impl RerunClient {
    pub fn new(config: StreamingConfig) -> Self {
        Self { config }
    }

    pub async fn connect(&mut self) -> Result<()> {
        tracing::info!("Connecting to Rerun server at: {}", self.config.server_url);
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        tracing::info!("Disconnecting from Rerun server");
        Ok(())
    }
}
