use crate::models::DeviceCommand;
use anyhow::Result;

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(command: DeviceCommand) -> Result<()> {
        match command {
            DeviceCommand::Configure(_config) => {
                tracing::info!("Configuring device");
                Ok(())
            }
            DeviceCommand::StartCollection => {
                tracing::info!("Starting data collection");
                Ok(())
            }
            DeviceCommand::StopCollection => {
                tracing::info!("Stopping data collection");
                Ok(())
            }
            DeviceCommand::Query => {
                tracing::info!("Querying device status");
                Ok(())
            }
            DeviceCommand::Disconnect => {
                tracing::info!("Disconnecting from device");
                Ok(())
            }
        }
    }
}
