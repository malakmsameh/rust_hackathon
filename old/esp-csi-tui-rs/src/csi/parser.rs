use regex::Regex;
use crate::models::CSIPacket;

pub struct CSIParser {
    mac_regex: Regex,
    rssi_regex: Regex,
    channel_regex: Regex,
    csi_data_regex: Regex,
}

impl CSIParser {
    pub fn new() -> Self {
        Self {
            mac_regex: Regex::new(r"mac:\s+([0-9a-fA-F:]+)").unwrap(),
            rssi_regex: Regex::new(r"rssi:\s+(-?\d+)").unwrap(),
            channel_regex: Regex::new(r"channel:\s+(\d+)").unwrap(),
            csi_data_regex: Regex::new(r"csi raw data:\s*\[([^\]]+)\]").unwrap(),
        }
    }

    /// Parse a multi-line CSI output block
    pub fn parse_packet(&self, output: &str) -> Option<CSIPacket> {
        let mut packet = CSIPacket {
            mac: self.extract_field(&self.mac_regex, output)?,
            rssi: output
                .lines()
                .find(|l| l.contains("rssi:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            channel: output
                .lines()
                .find(|l| l.contains("channel:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            timestamp: output
                .lines()
                .find(|l| l.contains("timestamp:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            rate: output
                .lines()
                .find(|l| l.contains("rate:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            noise_floor: output
                .lines()
                .find(|l| l.contains("noise floor:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            sig_len: output
                .lines()
                .find(|l| l.contains("sig len:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            rx_state: output
                .lines()
                .find(|l| l.contains("rx state:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            secondary_channel: output
                .lines()
                .find(|l| l.contains("secondary channel:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            sgi: output
                .lines()
                .find(|l| l.contains("sgi:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            ant: output
                .lines()
                .find(|l| l.contains("ant:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            ampdu_cnt: output
                .lines()
                .find(|l| l.contains("ampdu cnt:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            sig_mode: output
                .lines()
                .find(|l| l.contains("sig_mode:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            mcs: output
                .lines()
                .find(|l| l.contains("mcs:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            cwb: output
                .lines()
                .find(|l| l.contains("cwb:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            smoothing: output
                .lines()
                .find(|l| l.contains("smoothing:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            not_sounding: output
                .lines()
                .find(|l| l.contains("not sounding:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            aggregation: output
                .lines()
                .find(|l| l.contains("aggregation:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            stbc: output
                .lines()
                .find(|l| l.contains("stbc:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            fec_coding: output
                .lines()
                .find(|l| l.contains("fec coding:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            data_length: output
                .lines()
                .find(|l| l.contains("data length:"))?
                .split(':')
                .nth(1)?
                .trim()
                .parse()
                .ok()?,
            csi_data: Vec::new(),
        };

        // Parse CSI data array
        packet.csi_data = self.extract_csi_array(output)?;

        Some(packet)
    }

    fn extract_field(&self, regex: &Regex, text: &str) -> Option<String> {
        regex.captures(text).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
    }

    fn extract_csi_array(&self, output: &str) -> Option<Vec<i32>> {
        let data_line = output
            .lines()
            .find(|l| l.starts_with("[0,") || l.starts_with("[-"))?;

        // Parse array of numbers: [1, 2, 3, ...]
        let cleaned = data_line
            .trim_matches(|c| c == '[' || c == ']')
            .trim();

        let values: Vec<i32> = cleaned
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if values.is_empty() {
            None
        } else {
            Some(values)
        }
    }
}

impl Default for CSIParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let sample_output = r#"
mac: 40:E1:E4:1F:81:C6
rssi: -61
rate: 11
noise floor: 161
channel: 11
timestamp: 3195366537
sig len: 100
rx state: 0
secondary channel: 0
sgi: 0
ant: 0
ampdu cnt: 0
sig_mode: 1
mcs: 7
cwb: 0
smoothing: 1
not sounding: 1
aggregation: 1
stbc: 1
fec coding: 0
sig_len: 100
data length: 384
csi raw data:
[-14, 5, -14, 6, -14, 7]
"#;

        let parser = CSIParser::new();
        let packet = parser.parse_packet(sample_output);

        assert!(packet.is_some());
        let pkt = packet.unwrap();
        assert_eq!(pkt.mac, "40:E1:E4:1F:81:C6");
        assert_eq!(pkt.rssi, -61);
        assert_eq!(pkt.channel, 11);
    }
}
