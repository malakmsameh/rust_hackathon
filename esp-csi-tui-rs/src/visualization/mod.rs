pub mod renderer;
pub mod charts;

use crate::models::CsiMeasurement;
use anyhow::Result;
use std::collections::HashMap;

pub struct DataProcessor;

impl DataProcessor {
    pub fn magnitude_spectrum(measurements: &[CsiMeasurement]) -> Result<Vec<f32>> {
        if measurements.is_empty() {
            return Ok(Vec::new());
        }

        let latest = &measurements[measurements.len() - 1];
        Ok(latest
            .subcarrier_data
            .iter()
            .map(|c| c.magnitude())
            .collect())
    }

    pub fn phase_spectrum(measurements: &[CsiMeasurement]) -> Result<Vec<f32>> {
        if measurements.is_empty() {
            return Ok(Vec::new());
        }

        let latest = &measurements[measurements.len() - 1];
        Ok(latest
            .subcarrier_data
            .iter()
            .map(|c| c.phase())
            .collect())
    }

    pub fn heatmap_data(measurements: &[CsiMeasurement]) -> Result<Vec<Vec<f32>>> {
        let mut heatmap = Vec::new();
        
        for measurement in measurements {
            let magnitudes: Vec<f32> = measurement
                .subcarrier_data
                .iter()
                .map(|c| c.magnitude())
                .collect();
            heatmap.push(magnitudes);
        }

        Ok(heatmap)
    }

    pub fn statistics(measurements: &[CsiMeasurement]) -> Result<HashMap<String, f32>> {
        let mut stats = HashMap::new();

        if measurements.is_empty() {
            return Ok(stats);
        }

        let rssi_values: Vec<i8> = measurements.iter().map(|m| m.rssi).collect();
        stats.insert("avg_rssi".to_string(), rssi_values.iter().sum::<i8>() as f32 / rssi_values.len() as f32);
        stats.insert("min_rssi".to_string(), *rssi_values.iter().min().unwrap_or(&0) as f32);
        stats.insert("max_rssi".to_string(), *rssi_values.iter().max().unwrap_or(&0) as f32);

        Ok(stats)
    }
}
