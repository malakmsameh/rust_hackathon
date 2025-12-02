pub mod csv_storage;
pub mod rrd_storage;

use crate::models::CsiMeasurement;
use anyhow::Result;
use std::path::Path;

pub trait Storage {
    fn save(&self, measurements: &[CsiMeasurement], path: &Path) -> Result<()>;
    fn load(&self, path: &Path) -> Result<Vec<CsiMeasurement>>;
}
