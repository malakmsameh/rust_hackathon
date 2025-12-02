use crate::models::CSIPacket;
use serde::Serialize;
use std::path::Path;
use anyhow::Result;

#[derive(Serialize)]
pub struct CSIRecord {
    pub timestamp_us: u64,
    pub mac_address: String,
    pub rssi_dbm: i32,
    pub channel: u8,
    pub rate_mbps: u8,
    pub signal_length: u16,
    pub subcarrier_count: usize,
    pub amplitude_peak: f32,
    pub amplitude_mean: f32,
}

impl From<&CSIPacket> for CSIRecord {
    fn from(packet: &CSIPacket) -> Self {
        let amplitudes = packet.get_amplitude();
        
        let peak = amplitudes
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        let mean = if !amplitudes.is_empty() {
            amplitudes.iter().sum::<f32>() / amplitudes.len() as f32
        } else {
            0.0
        };

        Self {
            timestamp_us: packet.timestamp,
            mac_address: packet.mac.clone(),
            rssi_dbm: packet.rssi,
            channel: packet.channel,
            rate_mbps: packet.rate,
            signal_length: packet.sig_len,
            subcarrier_count: packet.subcarrier_count(),
            amplitude_peak: peak,
            amplitude_mean: mean,
        }
    }
}

pub struct CSVExporter;

impl CSVExporter {
    pub fn export_packets(
        packets: &[CSIPacket],
        output_path: &Path,
    ) -> Result<()> {
        let mut writer = csv::Writer::from_path(output_path)?;

        for packet in packets {
            let record = CSIRecord::from(packet);
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn export_with_metadata(
        packets: &[CSIPacket],
        output_path: &Path,
        metadata: &[(String, String)],
    ) -> Result<()> {
        let mut file = std::fs::File::create(output_path)?;
        use std::io::Write;

        // Write metadata as comments
        for (key, value) in metadata {
            writeln!(file, "# {}: {}", key, value)?;
        }
        writeln!(file)?;

        let mut writer = csv::Writer::from_writer(file);

        for packet in packets {
            let record = CSIRecord::from(packet);
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_conversion() {
        let packet = CSIPacket {
            mac: "00:11:22:33:44:55".to_string(),
            rssi: -50,
            channel: 6,
            timestamp: 1000000,
            rate: 65,
            noise_floor: 161,
            sig_len: 100,
            rx_state: 0,
            secondary_channel: 0,
            sgi: 0,
            ant: 0,
            ampdu_cnt: 0,
            sig_mode: 1,
            mcs: 7,
            cwb: 0,
            smoothing: 1,
            not_sounding: 1,
            aggregation: 1,
            stbc: 1,
            fec_coding: 0,
            data_length: 384,
            csi_data: vec![10, 5, 20, 8, 15, 12],
        };

        let record = CSIRecord::from(&packet);
        assert_eq!(record.mac_address, "00:11:22:33:44:55");
        assert_eq!(record.rssi_dbm, -50);
        assert_eq!(record.channel, 6);
    }
}
