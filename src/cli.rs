use std::path::PathBuf;

use clap::Parser;

use crate::types::{DistributionPattern, OutputFormat};

/// ハイエトグラフ（降雨時間分布図）生成ツール
///
/// 降雨強度式のパラメータから交互ブロック法に基づくハイエトグラフを生成し、
/// PNG棒グラフおよびCSVデータとして出力する。
///
/// 使用例:
///   hyetograph-cli 0.75 5.411 1557.825 10 2
///   hyetograph-cli 0.75 5.411 1557.825 10 2 --pattern center --format both
#[derive(Parser, Debug)]
#[command(version, about, allow_negative_numbers = true)]
pub struct Cli {
    /// 降雨強度係数A（べき乗指数）
    pub a: f64,

    /// 降雨強度係数B（加算定数）
    pub b: f64,

    /// 降雨強度係数C（分子定数）
    pub c: f64,

    /// 計算時間刻み T [分]
    pub t: f64,

    /// 降雨継続時間 TT [時間]
    pub tt: f64,

    /// 雨量分布パターン (front: 前方集中, center: 中央集中, rear: 後方集中)
    #[arg(long, default_value = "center")]
    pub pattern: DistributionPattern,

    /// 出力ファイルパス
    #[arg(short, long, default_value = "hyetograph.png")]
    pub output: PathBuf,

    /// 出力形式 (png, csv, both)
    #[arg(short, long, default_value = "png")]
    pub format: OutputFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_required_args() {
        let cli =
            Cli::try_parse_from(["hyetograph-cli", "0.75", "5.411", "1557.825", "10", "2"])
                .unwrap();
        assert_eq!(cli.a, 0.75);
        assert_eq!(cli.b, 5.411);
        assert_eq!(cli.c, 1557.825);
        assert_eq!(cli.t, 10.0);
        assert_eq!(cli.tt, 2.0);
    }

    #[test]
    fn default_pattern_is_center() {
        let cli =
            Cli::try_parse_from(["hyetograph-cli", "0.75", "5.411", "1557.825", "10", "2"])
                .unwrap();
        assert_eq!(cli.pattern, DistributionPattern::Center);
    }

    #[test]
    fn default_output_is_hyetograph_png() {
        let cli =
            Cli::try_parse_from(["hyetograph-cli", "0.75", "5.411", "1557.825", "10", "2"])
                .unwrap();
        assert_eq!(cli.output, PathBuf::from("hyetograph.png"));
    }

    #[test]
    fn default_format_is_png() {
        let cli =
            Cli::try_parse_from(["hyetograph-cli", "0.75", "5.411", "1557.825", "10", "2"])
                .unwrap();
        assert_eq!(cli.format, OutputFormat::Png);
    }

    #[test]
    fn parse_all_options() {
        let cli = Cli::try_parse_from([
            "hyetograph-cli",
            "0.75",
            "5.411",
            "1557.825",
            "10",
            "2",
            "--pattern",
            "front",
            "--output",
            "output.csv",
            "--format",
            "both",
        ])
        .unwrap();
        assert_eq!(cli.pattern, DistributionPattern::Front);
        assert_eq!(cli.output, PathBuf::from("output.csv"));
        assert_eq!(cli.format, OutputFormat::Both);
    }

    #[test]
    fn missing_required_args_returns_error() {
        let result = Cli::try_parse_from(["hyetograph-cli", "0.75"]);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_pattern_returns_error() {
        let result = Cli::try_parse_from([
            "hyetograph-cli",
            "0.75",
            "5.411",
            "1557.825",
            "10",
            "2",
            "--pattern",
            "invalid",
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn help_flag_exits() {
        let result = Cli::try_parse_from(["hyetograph-cli", "--help"]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn version_flag_exits() {
        let result = Cli::try_parse_from(["hyetograph-cli", "--version"]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    }
}
