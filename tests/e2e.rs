use std::process::Command;

fn cargo_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_hyetograph-cli"))
}

fn base_args() -> Vec<&'static str> {
    vec!["0.75", "5.411", "1557.825", "10", "2"]
}

mod vba_compatibility {
    use super::*;
    use std::fs;

    const FRONT_EXPECTED: [f64; 12] = [
        141.179, 68.369, 46.819, 35.957, 29.354, 24.900, 21.684, 19.249, 17.339, 15.799, 14.530,
        13.465,
    ];

    const CENTER_EXPECTED: [f64; 12] = [
        13.465, 15.799, 19.249, 24.900, 35.957, 68.369, 141.179, 46.819, 29.354, 21.684, 17.339,
        14.530,
    ];

    const REAR_EXPECTED: [f64; 12] = [
        13.465, 14.530, 15.799, 17.339, 19.249, 21.684, 24.900, 29.354, 35.957, 46.819, 68.369,
        141.179,
    ];

    fn run_and_get_csv(pattern: &str) -> Vec<(f64, f64)> {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("output.csv");

        let mut args = base_args();
        args.extend(["--pattern", pattern, "--output"]);
        let csv_path_str = csv_path.to_str().unwrap().to_string();
        // We need to pass the path as a string
        let status = cargo_bin()
            .args(&args)
            .arg(&csv_path_str)
            .arg("--format")
            .arg("csv")
            .status()
            .expect("Failed to execute binary");

        assert!(status.success(), "Process should exit with code 0");

        let content = fs::read_to_string(&csv_path).expect("CSV file should exist");
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        reader
            .records()
            .map(|r| {
                let record = r.unwrap();
                let time: f64 = record[0].parse().unwrap();
                let intensity: f64 = record[1].parse().unwrap();
                (time, intensity)
            })
            .collect()
    }

    #[test]
    fn front_pattern_matches_vba() {
        let data = run_and_get_csv("front");
        assert_eq!(data.len(), 12);

        for (i, ((time, intensity), expected)) in data.iter().zip(FRONT_EXPECTED.iter()).enumerate()
        {
            assert_eq!(*time, 10.0 * (i + 1) as f64);
            let rounded = (intensity * 1000.0).round() / 1000.0;
            assert!(
                (rounded - expected).abs() < 1e-3,
                "Step {}: expected {}, got {} (rounded {})",
                i + 1,
                expected,
                intensity,
                rounded
            );
        }
    }

    #[test]
    fn center_pattern_matches_vba() {
        let data = run_and_get_csv("center");
        assert_eq!(data.len(), 12);

        for (i, ((_, intensity), expected)) in data.iter().zip(CENTER_EXPECTED.iter()).enumerate() {
            let rounded = (intensity * 1000.0).round() / 1000.0;
            assert!(
                (rounded - expected).abs() < 1e-3,
                "Step {}: expected {}, got {} (rounded {})",
                i + 1,
                expected,
                intensity,
                rounded
            );
        }
    }

    #[test]
    fn rear_pattern_matches_vba() {
        let data = run_and_get_csv("rear");
        assert_eq!(data.len(), 12);

        for (i, ((_, intensity), expected)) in data.iter().zip(REAR_EXPECTED.iter()).enumerate() {
            let rounded = (intensity * 1000.0).round() / 1000.0;
            assert!(
                (rounded - expected).abs() < 1e-3,
                "Step {}: expected {}, got {} (rounded {})",
                i + 1,
                expected,
                intensity,
                rounded
            );
        }
    }

    #[test]
    fn png_output_generated_for_all_patterns() {
        for pattern in ["front", "center", "rear"] {
            let dir = tempfile::tempdir().unwrap();
            let png_path = dir.path().join("output.png");

            let mut args = base_args();
            args.extend(["--pattern", pattern, "--output"]);

            let status = cargo_bin()
                .args(&args)
                .arg(png_path.to_str().unwrap())
                .arg("--format")
                .arg("png")
                .status()
                .expect("Failed to execute binary");

            assert!(
                status.success(),
                "Pattern {}: should exit with code 0",
                pattern
            );
            assert!(
                png_path.exists(),
                "Pattern {}: PNG file should exist",
                pattern
            );
            let metadata = std::fs::metadata(&png_path).unwrap();
            assert!(
                metadata.len() > 0,
                "Pattern {}: PNG file should not be empty",
                pattern
            );
        }
    }
}

mod error_cases {
    use super::*;

    #[test]
    fn negative_parameter_returns_error() {
        let output = cargo_bin()
            .args(["-0.5", "5.411", "1557.825", "10", "2"])
            .output()
            .expect("Failed to execute binary");

        assert!(!output.status.success(), "Should fail with negative A");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Parameter A") || stderr.contains("positive"),
            "stderr should mention parameter: {}",
            stderr
        );
    }

    #[test]
    fn non_integer_nt_returns_error() {
        let output = cargo_bin()
            .args(["0.75", "5.411", "1557.825", "7", "1"])
            .output()
            .expect("Failed to execute binary");

        assert!(!output.status.success(), "Should fail with non-integer NT");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("integer"),
            "stderr should mention integer: {}",
            stderr
        );
    }

    #[test]
    fn nonexistent_output_dir_returns_error() {
        let output = cargo_bin()
            .args([
                "0.75",
                "5.411",
                "1557.825",
                "10",
                "2",
                "--output",
                "/nonexistent/dir/out.png",
            ])
            .output()
            .expect("Failed to execute binary");

        assert!(
            !output.status.success(),
            "Should fail with nonexistent output dir"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("does not exist"),
            "stderr should mention directory: {}",
            stderr
        );
    }

    #[test]
    fn help_flag_succeeds() {
        let output = cargo_bin()
            .arg("--help")
            .output()
            .expect("Failed to execute binary");

        assert!(output.status.success(), "--help should exit with code 0");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Usage"), "Should show usage: {}", stdout);
    }

    #[test]
    fn version_flag_succeeds() {
        let output = cargo_bin()
            .arg("--version")
            .output()
            .expect("Failed to execute binary");

        assert!(output.status.success(), "--version should exit with code 0");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("hyetograph-cli"),
            "Should show version: {}",
            stdout
        );
    }
}
