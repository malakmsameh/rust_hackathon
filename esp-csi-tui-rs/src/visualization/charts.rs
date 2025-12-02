#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub show_grid: bool,
    pub auto_scale: bool,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            title: "CSI Data".to_string(),
            x_label: "Subcarrier Index".to_string(),
            y_label: "Magnitude (dB)".to_string(),
            show_grid: true,
            auto_scale: true,
        }
    }
}

#[derive(Debug)]
pub enum ChartType {
    MagnitudeSpectrum,
    PhaseSpectrum,
    Heatmap,
    TimeSeries,
    Complex3D,
}

pub struct ChartBuilder;

impl ChartBuilder {
    pub fn create_config(chart_type: ChartType) -> ChartConfig {
        match chart_type {
            ChartType::MagnitudeSpectrum => ChartConfig {
                title: "CSI Magnitude Spectrum".to_string(),
                x_label: "Subcarrier Index".to_string(),
                y_label: "Magnitude (dB)".to_string(),
                ..Default::default()
            },
            ChartType::PhaseSpectrum => ChartConfig {
                title: "CSI Phase Spectrum".to_string(),
                x_label: "Subcarrier Index".to_string(),
                y_label: "Phase (radians)".to_string(),
                ..Default::default()
            },
            ChartType::Heatmap => ChartConfig {
                title: "CSI Time-Frequency Heatmap".to_string(),
                x_label: "Time".to_string(),
                y_label: "Subcarrier Index".to_string(),
                ..Default::default()
            },
            ChartType::TimeSeries => ChartConfig {
                title: "RSSI Over Time".to_string(),
                x_label: "Time".to_string(),
                y_label: "RSSI (dBm)".to_string(),
                ..Default::default()
            },
            ChartType::Complex3D => ChartConfig {
                title: "3D Complex CSI".to_string(),
                x_label: "Real".to_string(),
                y_label: "Imaginary".to_string(),
                ..Default::default()
            },
        }
    }
}
