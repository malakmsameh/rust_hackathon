use crate::models::CsiMeasurement;
use anyhow::{Result, anyhow};
use std::path::Path;

pub struct RrdStorage;

impl RrdStorage {
    pub fn save(measurements: &[CsiMeasurement], path: &Path) -> Result<()> {
        let rrd_data = serde_json::json!({
            "format": "rerun_recording",
            "version": 1,
            "measurements": measurements,
        });

        std::fs::write(path, rrd_data.to_string())?;
        tracing::info!("Saved {} measurements to RRD: {}", measurements.len(), path.display());
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Vec<CsiMeasurement>> {
        let content = std::fs::read_to_string(path)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;

        if let Some(measurements) = data.get("measurements").and_then(|v| v.as_array()) {
            let parsed: Vec<CsiMeasurement> = measurements
                .iter()
                .filter_map(|m| serde_json::from_value(m.clone()).ok())
                .collect();
            Ok(parsed)
        } else {
            Err(anyhow!("Invalid RRD format"))
        }
    }
}
