use std::path::PathBuf;

use anyhow::{bail, Result};

use crate::cli::Cli;
use crate::types::{DistributionPattern, RainfallParams};

/// バリデーション済みパラメータ
#[derive(Debug)]
pub struct ValidatedParams {
    pub rainfall_params: RainfallParams,
    pub pattern: DistributionPattern,
    pub output_config: OutputConfig,
}

/// 出力設定
#[derive(Debug)]
pub struct OutputConfig {
    pub output_path: PathBuf,
    pub format: crate::types::OutputFormat,
}

/// CLI引数のドメインバリデーション
pub fn validate(cli: &Cli) -> Result<ValidatedParams> {
    if cli.a <= 0.0 {
        bail!(
            "Parameter A must be positive (> 0), got {}. Valid range: A > 0",
            cli.a
        );
    }
    if cli.b <= 0.0 {
        bail!(
            "Parameter B must be positive (> 0), got {}. Valid range: B > 0",
            cli.b
        );
    }
    if cli.c <= 0.0 {
        bail!(
            "Parameter C must be positive (> 0), got {}. Valid range: C > 0",
            cli.c
        );
    }
    if cli.t <= 0.0 {
        bail!(
            "Parameter T must be positive (> 0), got {}. Valid range: T > 0",
            cli.t
        );
    }
    if cli.tt <= 0.0 {
        bail!(
            "Parameter TT must be positive (> 0), got {}. Valid range: TT > 0",
            cli.tt
        );
    }

    let nt = cli.tt * 60.0 / cli.t;
    if (nt - nt.round()).abs() > 1e-9 {
        bail!(
            "TT * 60 / T must be an integer. TT={}, T={} gives NT={:.4}, which is not an integer. \
             Adjust T or TT so that the duration divides evenly into time steps.",
            cli.tt,
            cli.t,
            nt
        );
    }

    if let Some(parent) = cli.output.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            bail!(
                "Output directory does not exist: {}",
                parent.display()
            );
        }
    }

    Ok(ValidatedParams {
        rainfall_params: RainfallParams {
            a: cli.a,
            b: cli.b,
            c: cli.c,
            t: cli.t,
            tt: cli.tt,
        },
        pattern: cli.pattern,
        output_config: OutputConfig {
            output_path: cli.output.clone(),
            format: cli.format,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn valid_cli() -> Cli {
        Cli::try_parse_from(["hyetograph-cli", "0.75", "5.411", "1557.825", "10", "2"]).unwrap()
    }

    fn cli_with_args(args: &[&str]) -> Cli {
        let mut full_args = vec!["hyetograph-cli"];
        full_args.extend_from_slice(args);
        Cli::try_parse_from(full_args).unwrap()
    }

    #[test]
    fn valid_params_pass_validation() {
        let cli = valid_cli();
        let result = validate(&cli);
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.rainfall_params.a, 0.75);
        assert_eq!(params.rainfall_params.tt, 2.0);
        assert_eq!(params.pattern, DistributionPattern::Center);
    }

    #[test]
    fn negative_a_fails() {
        let cli = cli_with_args(&["-0.5", "5.411", "1557.825", "10", "2"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("Parameter A"), "Error: {}", err);
        assert!(err.contains("positive"), "Error: {}", err);
    }

    #[test]
    fn zero_a_fails() {
        let cli = cli_with_args(&["0", "5.411", "1557.825", "10", "2"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("Parameter A"), "Error: {}", err);
    }

    #[test]
    fn negative_b_fails() {
        let cli = cli_with_args(&["0.75", "-1", "1557.825", "10", "2"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("Parameter B"), "Error: {}", err);
    }

    #[test]
    fn negative_c_fails() {
        let cli = cli_with_args(&["0.75", "5.411", "-100", "10", "2"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("Parameter C"), "Error: {}", err);
    }

    #[test]
    fn zero_t_fails() {
        let cli = cli_with_args(&["0.75", "5.411", "1557.825", "0", "2"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("Parameter T"), "Error: {}", err);
    }

    #[test]
    fn negative_tt_fails() {
        let cli = cli_with_args(&["0.75", "5.411", "1557.825", "10", "-1"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("Parameter TT"), "Error: {}", err);
    }

    #[test]
    fn non_integer_nt_fails() {
        // TT=1, T=7 -> NT = 60/7 ≈ 8.571
        let cli = cli_with_args(&["0.75", "5.411", "1557.825", "7", "1"]);
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("integer"), "Error: {}", err);
    }

    #[test]
    fn integer_nt_passes() {
        // TT=1, T=10 -> NT = 6 (integer)
        let cli = cli_with_args(&["0.75", "5.411", "1557.825", "10", "1"]);
        assert!(validate(&cli).is_ok());
    }

    #[test]
    fn nonexistent_output_dir_fails() {
        let cli = Cli::try_parse_from([
            "hyetograph-cli",
            "0.75",
            "5.411",
            "1557.825",
            "10",
            "2",
            "--output",
            "/nonexistent/dir/output.png",
        ])
        .unwrap();
        let err = validate(&cli).unwrap_err().to_string();
        assert!(err.contains("does not exist"), "Error: {}", err);
    }
}
