use crate::types::RainfallParams;

/// RKEISAN相当の降雨強度計算
/// 降雨強度式 K = C / ((T * I)^A + B) に基づき、各時間ステップの増分雨量を算出する。
/// 返却値は降雨強度の大きい順（R[0]が最大、降順）。
pub fn calculate(params: &RainfallParams) -> Vec<f64> {
    let nt = (params.tt * 60.0 / params.t) as usize;
    let mut increments = Vec::with_capacity(nt);
    let mut zk: f64 = 0.0;

    for i in 1..=nt {
        let ti = params.t * i as f64;
        let k = params.c / (ti.powf(params.a) + params.b);
        let cumulative = k * i as f64;
        let r = cumulative - zk;
        increments.push(r);
        zk = cumulative;
    }

    increments
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vba_test_params() -> RainfallParams {
        RainfallParams {
            a: 0.75,
            b: 5.411,
            c: 1557.825,
            t: 10.0,
            tt: 2.0,
        }
    }

    /// VBA期待値（小数3桁に丸めた値）
    const VBA_EXPECTED: [f64; 12] = [
        141.179, 68.369, 46.819, 35.957, 29.354, 24.900, 21.684, 19.249, 17.339, 15.799, 14.530,
        13.465,
    ];

    #[test]
    fn calculate_returns_correct_number_of_steps() {
        let params = vba_test_params();
        let result = calculate(&params);
        assert_eq!(result.len(), 12);
    }

    #[test]
    fn calculate_matches_vba_output() {
        let params = vba_test_params();
        let result = calculate(&params);

        for (i, (actual, expected)) in result.iter().zip(VBA_EXPECTED.iter()).enumerate() {
            let rounded = (actual * 1000.0).round() / 1000.0;
            assert!(
                (rounded - expected).abs() < 1e-3,
                "Step {}: expected {}, got {} (rounded: {})",
                i + 1,
                expected,
                actual,
                rounded
            );
        }
    }

    #[test]
    fn calculate_values_are_in_descending_order() {
        let params = vba_test_params();
        let result = calculate(&params);

        for i in 1..result.len() {
            assert!(
                result[i - 1] > result[i],
                "R({}) = {} should be > R({}) = {}",
                i,
                result[i - 1],
                i + 1,
                result[i]
            );
        }
    }

    #[test]
    fn calculate_all_values_are_positive() {
        let params = vba_test_params();
        let result = calculate(&params);

        for (i, val) in result.iter().enumerate() {
            assert!(*val > 0.0, "R({}) = {} should be positive", i + 1, val);
        }
    }

    #[test]
    fn calculate_sum_equals_total_cumulative() {
        let params = vba_test_params();
        let result = calculate(&params);

        // 増分雨量の合計 == 最終ステップの累計値 K(NT) * NT
        let nt = 12;
        let t_nt = params.t * nt as f64;
        let k_nt = params.c / (t_nt.powf(params.a) + params.b);
        let expected_total = k_nt * nt as f64;

        let actual_total: f64 = result.iter().sum();
        assert!(
            (actual_total - expected_total).abs() < 1e-10,
            "Sum {} should equal cumulative total {}",
            actual_total,
            expected_total
        );
    }
}
