use clap::ValueEnum;
use serde::Serialize;

/// 降雨強度計算のパラメータ
#[derive(Debug)]
pub struct RainfallParams {
    /// べき乗指数
    pub a: f64,
    /// 加算定数
    pub b: f64,
    /// 分子定数
    pub c: f64,
    /// 計算時間刻み[分]
    pub t: f64,
    /// 降雨継続時間[時間]
    pub tt: f64,
}

/// ハイエトグラフの1エントリ（経過時間と降雨強度のペア）
#[derive(Debug, Clone, Serialize)]
pub struct HyetographEntry {
    /// 経過時間[分]（T * index）
    pub time_minutes: f64,
    /// 降雨強度[mm/h]
    #[serde(rename = "intensity_mm_per_h")]
    pub intensity: f64,
}

/// 雨量分布パターン
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DistributionPattern {
    /// 前方集中型（パターン1）
    Front,
    /// 中央集中型（パターン2）
    Center,
    /// 後方集中型（パターン3）
    Rear,
}

impl Default for DistributionPattern {
    fn default() -> Self {
        Self::Center
    }
}

/// 出力形式
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// PNGのみ
    Png,
    /// CSVのみ
    Csv,
    /// PNG + CSV
    Both,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Png
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rainfall_params_holds_values() {
        let params = RainfallParams {
            a: 0.75,
            b: 5.411,
            c: 1557.825,
            t: 10.0,
            tt: 2.0,
        };
        assert_eq!(params.a, 0.75);
        assert_eq!(params.b, 5.411);
        assert_eq!(params.c, 1557.825);
        assert_eq!(params.t, 10.0);
        assert_eq!(params.tt, 2.0);
    }

    #[test]
    fn hyetograph_entry_holds_values() {
        let entry = HyetographEntry {
            time_minutes: 10.0,
            intensity: 141.179,
        };
        assert_eq!(entry.time_minutes, 10.0);
        assert_eq!(entry.intensity, 141.179);
    }

    #[test]
    fn hyetograph_entry_serializes_to_csv() {
        let entry = HyetographEntry {
            time_minutes: 10.0,
            intensity: 141.179,
        };
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&entry).unwrap();
        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        assert!(data.contains("time_minutes"));
        assert!(data.contains("intensity_mm_per_h"));
        assert!(data.contains("10"));
        assert!(data.contains("141.179"));
    }

    #[test]
    fn distribution_pattern_default_is_center() {
        assert_eq!(DistributionPattern::default(), DistributionPattern::Center);
    }

    #[test]
    fn distribution_pattern_has_three_variants() {
        let variants = DistributionPattern::value_variants();
        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn output_format_default_is_png() {
        assert_eq!(OutputFormat::default(), OutputFormat::Png);
    }

    #[test]
    fn output_format_has_three_variants() {
        let variants = OutputFormat::value_variants();
        assert_eq!(variants.len(), 3);
    }
}
