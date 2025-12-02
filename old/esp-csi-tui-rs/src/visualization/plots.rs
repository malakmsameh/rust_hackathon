use crate::models::CSIPacket;

pub struct ASCIIPlotter {
    width: usize,
    height: usize,
}

impl ASCIIPlotter {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Create ASCII plot of amplitude data (optimized for speed)
    pub fn plot_amplitude(&self, packets: &[CSIPacket]) -> String {
        if packets.is_empty() {
            return "No data".to_string();
        }

        let latest = &packets[packets.len() - 1];
        let amplitudes = latest.get_amplitude();

        if amplitudes.is_empty() {
            return "No amplitude data".to_string();
        }

        // Quick min/max without extra iterations
        let mut max_amp = f32::NEG_INFINITY;
        let mut min_amp = f32::INFINITY;
        
        for &amp in &amplitudes {
            if amp > max_amp {
                max_amp = amp;
            }
            if amp < min_amp {
                min_amp = amp;
            }
        }

        let range = (max_amp - min_amp).max(1.0);
        let height_factor = range / self.height as f32;
        
        // Pre-allocate with estimated capacity
        let mut plot = String::with_capacity((self.height + 2) * (amplitudes.len() + 2));

        // Plot from top to bottom
        for row in (0..self.height).rev() {
            let threshold = min_amp + (height_factor * row as f32);
            let lower_threshold = threshold - height_factor / 2.0;

            plot.push('│');

            // Render amplitude bars (simplified for speed)
            for &amp in &amplitudes {
                plot.push(if amp > threshold {
                    '█'
                } else if amp > lower_threshold {
                    '▄'
                } else {
                    ' '
                });
            }

            plot.push('│');
            plot.push('\n');
        }

        // Add bottom border (simple line)
        plot.push('└');
        for _ in 0..amplitudes.len().min(self.width) {
            plot.push('─');
        }
        plot.push('┘');

        plot
    }

    /// Create ASCII bar chart
    pub fn plot_bar_chart(&self, values: &[f32], label: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!("{}\n", label));

        let max_val = values
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b))
            .max(1.0);

        for (i, &val) in values.iter().enumerate().take(self.width) {
            let bar_len = ((val / max_val) * 30.0) as usize;
            output.push_str(&format!("{:3}: ", i));
            for _ in 0..bar_len {
                output.push('█');
            }
            output.push_str(&format!(" {:.2}\n", val));
        }

        output
    }
}

impl Default for ASCIIPlotter {
    fn default() -> Self {
        Self::new(50, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plotter_creation() {
        let plotter = ASCIIPlotter::new(80, 20);
        assert_eq!(plotter.width, 80);
        assert_eq!(plotter.height, 20);
    }
}
