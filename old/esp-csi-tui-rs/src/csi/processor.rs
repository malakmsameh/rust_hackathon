use crate::models::CSIPacket;
use crate::csi::CSIParser;

pub struct CSIProcessor {
    parser: CSIParser,
}

impl CSIProcessor {
    pub fn new() -> Self {
        Self {
            parser: CSIParser::new(),
        }
    }

    /// Process raw output and extract CSI packets
    pub fn process_output(&self, output: &str) -> Vec<CSIPacket> {
        let mut packets = Vec::new();

        // Split by logical blocks (empty lines or specific markers)
        let blocks: Vec<&str> = output.split("\n\n").collect();

        for block in blocks {
            if let Some(packet) = self.parser.parse_packet(block) {
                packets.push(packet);
            }
        }

        packets
    }
}

impl Default for CSIProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_single_packet() {
        let sample = r#"mac: 40:E1:E4:1F:81:C6
rssi: -61
channel: 11
timestamp: 1000000
rate: 11
noise floor: 161
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
data length: 384
csi raw data:
[-14, 5, -14, 6, -14, 7]"#;

        let processor = CSIProcessor::new();
        let packets = processor.process_output(sample);
        
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].mac, "40:E1:E4:1F:81:C6");
    }
}
