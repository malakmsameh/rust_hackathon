use crate::models::CsiMeasurement;
use anyhow::Result;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct CsvRecord {
    timestamp: String,
    channel: u8,
    bandwidth: u16,
    rssi: i8,
    noise_floor: i8,
    subcarrier_index: usize,
    real: f32,
    imag: f32,
    magnitude: f32,
    phase: f32,
}

pub struct CsvStorage;

impl CsvStorage {
    pub fn save(measurements: &[CsiMeasurement], path: &Path) -> Result<()> {
        let mut writer = Writer::from_path(path)?;

        for measurement in measurements {
            for (idx, subcarrier) in measurement.subcarrier_data.iter().enumerate() {
                let record = CsvRecord {
                    timestamp: measurement.timestamp.to_rfc3339(),
                    channel: measurement.channel,
                    bandwidth: measurement.bandwidth,
                    rssi: measurement.rssi,
                    noise_floor: measurement.noise_floor,
                    subcarrier_index: idx,
                    real: subcarrier.real,
                    imag: subcarrier.imag,
                    magnitude: subcarrier.magnitude(),
                    phase: subcarrier.phase(),
                };

                writer.serialize(record)?;
            }
        }

        writer.flush()?;
        tracing::info!("Saved {} measurements to CSV: {}", measurements.len(), path.display());
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Vec<CsiMeasurement>> {
        let reader = csv::Reader::from_path(path)?;
        let measurements = Vec::new();

        for result in reader.into_records() {
            let _record = result?;
        }

        Ok(measurements)
    }
}
