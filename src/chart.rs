use std::path::Path;

use anyhow::{Context, Result};
use plotters::prelude::*;

use crate::types::HyetographEntry;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

/// ハイエトグラフをPNG棒グラフとして描画する
pub fn render(data: &[HyetographEntry], output_path: &Path, time_step: f64) -> Result<()> {
    let max_intensity = data.iter().map(|e| e.intensity).fold(0.0_f64, f64::max);
    let max_time = data.last().map(|e| e.time_minutes).unwrap_or(0.0);

    let root = BitMapBackend::new(output_path, (DEFAULT_WIDTH, DEFAULT_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)
        .with_context(|| format!("Failed to initialize chart at {}", output_path.display()))?;

    let y_max = max_intensity * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Hyetograph", ("sans-serif", 24).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..max_time, 0.0..y_max)
        .with_context(|| "Failed to build chart")?;

    chart
        .configure_mesh()
        .x_desc("Time [min]")
        .y_desc("Intensity [mm/h]")
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .draw()
        .with_context(|| "Failed to draw mesh")?;

    chart
        .draw_series(data.iter().map(|entry| {
            let x0 = entry.time_minutes - time_step;
            let x1 = entry.time_minutes;
            Rectangle::new([(x0, 0.0), (x1, entry.intensity)], BLUE.filled())
        }))
        .with_context(|| "Failed to draw bars")?;

    root.present()
        .with_context(|| format!("Failed to save chart to {}", output_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<HyetographEntry> {
        vec![
            HyetographEntry {
                time_minutes: 10.0,
                intensity: 13.465,
            },
            HyetographEntry {
                time_minutes: 20.0,
                intensity: 68.369,
            },
            HyetographEntry {
                time_minutes: 30.0,
                intensity: 141.179,
            },
        ]
    }

    #[test]
    fn render_creates_png_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_chart.png");

        render(&sample_data(), &path, 10.0).unwrap();

        assert!(path.exists(), "PNG file should be created");
        let metadata = std::fs::metadata(&path).unwrap();
        assert!(metadata.len() > 0, "PNG file should not be empty");
    }

    #[test]
    fn render_nonexistent_dir_fails() {
        let path = Path::new("/nonexistent/dir/chart.png");
        let result = render(&sample_data(), path, 10.0);
        assert!(result.is_err());
    }
}
