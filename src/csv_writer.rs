use std::path::Path;

use anyhow::{Context, Result};

use crate::types::HyetographEntry;

/// ハイエトグラフデータをCSVファイルに出力する
pub fn write(data: &[HyetographEntry], output_path: &Path) -> Result<()> {
    let mut wtr =
        csv::Writer::from_path(output_path).with_context(|| {
            format!(
                "Failed to create CSV file: {}",
                output_path.display()
            )
        })?;

    for entry in data {
        wtr.serialize(entry).with_context(|| {
            format!("Failed to write CSV record to {}", output_path.display())
        })?;
    }

    wtr.flush().with_context(|| {
        format!("Failed to flush CSV file: {}", output_path.display())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
    fn writes_csv_with_header_and_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_output.csv");

        write(&sample_data(), &path).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        assert_eq!(lines[0], "time_minutes,intensity_mm_per_h");
        assert_eq!(lines[1], "10.0,13.465");
        assert_eq!(lines[2], "20.0,68.369");
        assert_eq!(lines[3], "30.0,141.179");
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn nonexistent_parent_dir_returns_error() {
        let path = Path::new("/nonexistent/dir/output.csv");
        let result = write(&sample_data(), path);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Failed to create CSV file"), "Error: {}", err);
    }
}
