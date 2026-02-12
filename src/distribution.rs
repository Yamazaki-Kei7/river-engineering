use crate::types::{DistributionPattern, HyetographEntry};

/// 増分雨量を時間軸上に配置する
/// increments: 降順の増分雨量（R[0]が最大）
/// pattern: 配置パターン
/// t: 計算時間刻み[分]
/// 返却: 時系列順に並んだHyetographEntry配列
pub fn arrange(increments: &[f64], pattern: DistributionPattern, t: f64) -> Vec<HyetographEntry> {
    let nt = increments.len();
    let mut arranged = vec![0.0; nt];

    match pattern {
        DistributionPattern::Front => {
            // 前方集中型: 降順のまま先頭から配置
            arranged.copy_from_slice(increments);
        }
        DistributionPattern::Rear => {
            // 後方集中型: 逆順（昇順）で先頭から配置
            for (i, val) in increments.iter().rev().enumerate() {
                arranged[i] = *val;
            }
        }
        DistributionPattern::Center => {
            // 中央集中型: VBA Case 2 と同一ロジック
            // 0-based index j に対して:
            //   j偶数 (I奇数): pos = nt/2 + j/2
            //   j奇数 (I偶数): pos = nt/2 - 1 - j/2
            let center = nt / 2;
            for (j, val) in increments.iter().enumerate() {
                let pos = if j % 2 == 0 {
                    center + j / 2
                } else {
                    center - 1 - j / 2
                };
                arranged[pos] = *val;
            }
        }
    }

    arranged
        .into_iter()
        .enumerate()
        .map(|(i, intensity)| HyetographEntry {
            time_minutes: t * (i + 1) as f64,
            intensity,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INCREMENTS: [f64; 12] = [
        141.179, 68.369, 46.819, 35.957, 29.354, 24.900, 21.684, 19.249, 17.339, 15.799, 14.530,
        13.465,
    ];

    const T: f64 = 10.0;

    #[test]
    fn front_pattern_places_descending_from_start() {
        let result = arrange(&INCREMENTS, DistributionPattern::Front, T);
        let expected = [
            141.179, 68.369, 46.819, 35.957, 29.354, 24.900, 21.684, 19.249, 17.339, 15.799,
            14.530, 13.465,
        ];

        assert_eq!(result.len(), 12);
        for (i, (entry, exp)) in result.iter().zip(expected.iter()).enumerate() {
            assert_eq!(entry.time_minutes, T * (i + 1) as f64);
            assert!(
                (entry.intensity - exp).abs() < 1e-10,
                "pos {}: expected {}, got {}",
                i,
                exp,
                entry.intensity
            );
        }
    }

    #[test]
    fn center_pattern_matches_vba_case2() {
        let result = arrange(&INCREMENTS, DistributionPattern::Center, T);
        let expected = [
            13.465, 15.799, 19.249, 24.900, 35.957, 68.369, 141.179, 46.819, 29.354, 21.684,
            17.339, 14.530,
        ];

        assert_eq!(result.len(), 12);
        for (i, (entry, exp)) in result.iter().zip(expected.iter()).enumerate() {
            assert_eq!(entry.time_minutes, T * (i + 1) as f64);
            assert!(
                (entry.intensity - exp).abs() < 1e-10,
                "pos {}: expected {}, got {}",
                i,
                exp,
                entry.intensity
            );
        }
    }

    #[test]
    fn rear_pattern_places_ascending_from_start() {
        let result = arrange(&INCREMENTS, DistributionPattern::Rear, T);
        let expected = [
            13.465, 14.530, 15.799, 17.339, 19.249, 21.684, 24.900, 29.354, 35.957, 46.819,
            68.369, 141.179,
        ];

        assert_eq!(result.len(), 12);
        for (i, (entry, exp)) in result.iter().zip(expected.iter()).enumerate() {
            assert_eq!(entry.time_minutes, T * (i + 1) as f64);
            assert!(
                (entry.intensity - exp).abs() < 1e-10,
                "pos {}: expected {}, got {}",
                i,
                exp,
                entry.intensity
            );
        }
    }

    #[test]
    fn sum_is_preserved_across_all_patterns() {
        let original_sum: f64 = INCREMENTS.iter().sum();

        for pattern in [
            DistributionPattern::Front,
            DistributionPattern::Center,
            DistributionPattern::Rear,
        ] {
            let result = arrange(&INCREMENTS, pattern, T);
            let arranged_sum: f64 = result.iter().map(|e| e.intensity).sum();
            assert!(
                (original_sum - arranged_sum).abs() < 1e-10,
                "Pattern {:?}: sum {} != original {}",
                pattern,
                arranged_sum,
                original_sum
            );
        }
    }

    #[test]
    fn time_minutes_is_ascending() {
        for pattern in [
            DistributionPattern::Front,
            DistributionPattern::Center,
            DistributionPattern::Rear,
        ] {
            let result = arrange(&INCREMENTS, pattern, T);
            for i in 1..result.len() {
                assert!(result[i].time_minutes > result[i - 1].time_minutes);
            }
        }
    }
}
