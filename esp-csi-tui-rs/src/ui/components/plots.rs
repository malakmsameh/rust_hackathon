use crate::models::CsiMeasurement;

pub enum PlotType {
    TwoDimensional,
    ThreeDimensional,
    Heatmap,
    ColorDomain,
    TimeSeries,
}

pub struct PlotRenderer {
    plot_type: PlotType,
}

impl PlotRenderer {
    pub fn new(plot_type: PlotType) -> Self {
        Self { plot_type }
    }

    pub fn render_data(&self, _measurements: &[CsiMeasurement]) -> String {
        match self.plot_type {
            PlotType::TwoDimensional => "2D Plot View".to_string(),
            PlotType::ThreeDimensional => "3D Plot View".to_string(),
            PlotType::Heatmap => "Heatmap View".to_string(),
            PlotType::ColorDomain => "Color Domain View".to_string(),
            PlotType::TimeSeries => "Time Series View".to_string(),
        }
    }
}
