use crate::models::CsiMeasurement;
use plotly::Plot;

pub struct Renderer;

impl Renderer {
    pub fn create_magnitude_plot(_measurements: &[CsiMeasurement]) -> Plot {
        Plot::new()
    }

    pub fn create_phase_plot(_measurements: &[CsiMeasurement]) -> Plot {
        Plot::new()
    }

    pub fn create_time_series_plot(_measurements: &[CsiMeasurement]) -> Plot {
        Plot::new()
    }
}
