#[cfg(test)]
mod tests {
    use crate::utils::check_price_fluctuation;

    #[test]
    fn test_price_up_first() {
        let prices = vec![
            100_000, // 100.000
            105_000, // 105.000 (5% increase)
            95_000 // 95.000
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_price_down_first() {
        let prices = vec![
            100_000, // 100.000
            95_000, // 95.000 (5% decrease)
            105_000 // 105.000
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(false));
    }

    #[test]
    fn test_no_fluctuation() {
        let prices = vec![
            100_000, // 100.000
            101_000, // 101.000
            102_000, // 102.000
            103_000 // 103.000
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), None);
    }

    #[test]
    fn test_invalid_start_index() {
        let prices = vec![100_000, 105_000];
        assert_eq!(check_price_fluctuation(&prices, 5, 5, 3, 0), None);
    }

    #[test]
    fn test_exact_threshold_up() {
        let prices = vec![
            100_000, // 100.000
            105_000 // 105.000 (exact 5% increase)
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_exact_threshold_down() {
        let prices = vec![
            100_000, // 100.000
            95_000 // 95.000 (exact 5% decrease)
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(false));
    }

    /// 2. Testing with Different `price_decimals` and `percentage_decimals`
    #[test]
    fn test_different_decimals_up_first() {
        let prices = vec![
            100_000_000, // 100.000000 (price_decimals = 6)
            105_000_000, // 105.000000 (5% increase)
            99_000_000 // 99.000000
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 500, 6, 2), // 500 with percentage_decimals = 2 represents 5.00%
            Some(true)
        );
    }

    #[test]
    fn test_different_decimals_down_first() {
        let prices = vec![
            200_000_000, // 200.000000
            190_000_000, // 190.000000
            180_000_000 // 180.000000 (10% decrease)
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 1000, 6, 2), // 1000 with percentage_decimals = 2 represents 10.00%
            Some(false)
        );
    }

    #[test]
    fn test_high_precision_no_fluctuation() {
        let prices = vec![
            100_000_000, // 100.000000
            100_499_999, // 100.499999 (just below 0.5% increase)
            100_500_001, // 100.500001 (exactly 0.5000001% increase)
            99_500_000 // 99.500000 (exact 0.5% decrease)
        ];
        // Threshold set at 0.5% with percentage_decimals = 6
        assert_eq!(
            check_price_fluctuation(&prices, 0, 50000, 6, 5), // 50000 represents 0.50000%
            Some(true) // Because 100_500_001 > 100.000000 * 1.005
        );

        // Another check where decrease meets exactly
        let prices_decrease = vec![
            100_000_000, // 100.000000
            99_500_000 // 99.500000 (exact 0.5% decrease)
        ];
        assert_eq!(check_price_fluctuation(&prices_decrease, 0, 50000, 6, 5), Some(false));
    }

    #[test]
    fn test_large_values_up_first() {
        let prices = vec![
            1_000_000_000_000, // 1,000,000,000.000 (price_decimals = 3)
            1_050_000_000_000, // 1,050,000,000.000 (5% increase)
            950_000_000_000 // 950,000,000.000 (5% decrease)
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_large_values_down_first() {
        let prices = vec![
            2_000_000_000_000, // 2,000,000,000.000
            1_900_000_000_000, // 1,900,000,000.000
            1_850_000_000_000 // 1,850,000,000.000 (7.5% decrease)
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 7, 3, 0), Some(false));
    }

    #[test]
    fn test_fluctuation_just_below_threshold() {
        let prices = vec![
            100_000, // 100.000
            104_999, // 104.999 (4.999%)
            105_001 // 105.001 (5.0001%)
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_fluctuation_just_above_threshold_down_first() {
        let prices = vec![
            100_000, // 100.000
            95_001, // 95.001 (4.999% decrease)
            94_999 // 94.999 (5.001% decrease)
        ];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(false));
    }

    #[test]
    fn test_mixed_decimals_up_first() {
        let prices = vec![
            123_456_789, // 123.456789 (price_decimals = 6)
            130_124_777, // ~130.124777 (~5% increase)
            121_333_100 // ~121.333100
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 500, 6, 3), // 500 with percentage_decimals = 3 represents 0.500%
            Some(true)
        );
    }

    #[test]
    fn test_mixed_decimals_down_first() {
        let prices = vec![
            250_000_000, // 250.000000 (price_decimals = 6)
            237_500_000, // 237.500000 (~5% decrease)
            240_000_000 // 240.000000
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 500, 6, 3), // 500 represents 0.500%
            Some(false)
        );
    }

    #[test]
    fn test_minimal_prices_high_precision() {
        let prices = vec![
            1, // 0.000001 (price_decimals = 6)
            1, // 0.000001
            1, // 0.000001
            2 // 0.000002 (100% increase)
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 100, 6, 2), // 100 with percentage_decimals = 2 => 1.00%
            Some(true)
        );
    }

    #[test]
    fn test_no_fluctuation_high_percentage_decimals() {
        let prices = vec![
            100_000, // 100.000
            100_499, // 100.499 (0.499%)
            100_500, // 100.500 (0.500%)
            99_500, // 99.500 (0.500%)
            99_499 // 99.499 (0.501%)
        ];
        // Setting percentage_decimals = 3 (0.500%)
        assert_eq!(
            check_price_fluctuation(&prices, 0, 500, 3, 3),
            Some(true) // 100_500 meets exactly 0.500% increase
        );

        // Check for decrease
        let prices_decrease = vec![
            100_000, // 100.000
            99_500 // 99.500 (0.500% decrease)
        ];
        assert_eq!(check_price_fluctuation(&prices_decrease, 0, 500, 3, 3), Some(false));
    }

    #[test]
    fn test_multiple_threshold_crossings_different_decimals() {
        let prices = vec![
            100_000_000, // 100.000000
            105_000_000, // 105.000000 (5% increase)
            94_750_000, // 94.750000 (5% decrease from 100.000000)
            110_000_000 // 110.000000 (10% increase)
        ];
        // Even though price first increased by 5%, then decreased by 5%, the function should return the first event
        assert_eq!(
            check_price_fluctuation(&prices, 0, 500, 6, 2), // 500 with percentage_decimals = 2 -> 5.00%
            Some(true)
        );
    }

    #[test]
    fn test_consecutive_fluctuations_below_threshold() {
        let prices = vec![
            100_000, // 100.000
            104_000, // 104.000 (4% increase)
            106_000, // 106.000 (6% increase) → meets upward threshold
            103_000 // 103.000 (3% decrease)
        ];
        // Threshold is 5%
        // The first fluctuation that meets/exceeds threshold is at 106_000 (6% increase)
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_fluctuation_exact_threshold_different_decimals() {
        let prices = vec![
            100_000_000, // 100.000000
            105_000_000, // 105.000000 (5.000000% increase)
            95_000_000 // 95.000000 (5.000000% decrease)
        ];
        // Threshold set at 5.00% with percentage_decimals = 2
        assert_eq!(
            check_price_fluctuation(&prices, 0, 500, 6, 2),
            Some(true) // First fluctuation is increase by exactly 5.00%
        );

        // Reverse order to test exact decrease first
        let prices_decrease_first = vec![
            100_000_000, // 100.000000
            95_000_000, // 95.000000 (5.000000% decrease)
            105_000_000 // 105.000000 (5.000000% increase)
        ];
        assert_eq!(check_price_fluctuation(&prices_decrease_first, 0, 500, 6, 2), Some(false));
    }

    #[test]
    fn test_multiple_crossings_mixed_order() {
        let prices = vec![
            100_000, // 100.000
            104_000, // 104.000 (4% increase)
            106_000, // 106.000 (6% increase) → meets upward threshold
            94_000 // 94.000 (6% decrease) → meets downward threshold
        ];
        // Should return true when 106_000 is reached
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_price_up_then_down() {
        let prices = vec![
            100_000, // 100.000
            105_000, // 105.000 (5% increase) → meets upward threshold
            95_000, // 95.000 (5% decrease from start) → meets downward threshold
            110_000 // 110.000 (additional increase)
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 5, 3, 0),
            Some(true) // Upward first
        );
    }

    #[test]
    fn test_price_down_then_up() {
        let prices = vec![
            100_000, // 100.000
            95_000, // 95.000 (5% decrease) → meets downward threshold
            105_000, // 105.000 (5% increase from start) → meets upward threshold
            90_000 // 90.000 (additional decrease)
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 5, 3, 0),
            Some(false) // Downward first
        );
    }

    #[test]
    fn test_fluctuations_hovering_threshold() {
        let prices = vec![
            100_000, // 100.000
            104_999, // 104.999 (4.999% increase - just below threshold)
            105_001, // 105.001 (5.001% increase) → meets upward threshold
            94_999 // 94.999 (5.001% decrease) → meets downward threshold
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 5, 3, 0),
            Some(true) // Upward meets first
        );
    }

    #[test]
    fn test_concurrent_threshold_crossings() {
        let prices = vec![
            100_000, // 100.000
            105_000, // 105.000 (5% increase) → meets upward threshold
            95_000, // 95.000 (5% decrease) → meets downward threshold
            105_000 // 105.000 (5% increase again)
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 5, 3, 0),
            Some(true) // Upward meets first
        );
    }

    #[test]
    fn test_empty_prices() {
        let prices: Vec<u64> = vec![];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), None);
    }

    #[test]
    fn test_single_price_point() {
        let prices = vec![100_000]; // 100.000
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), None);
    }

    #[test]
    fn test_all_prices_same() {
        let prices = vec![100_000, 100_000, 100_000];
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), None);
    }

    #[test]
    fn test_immediate_fluctuation() {
        let prices = vec![100_000, 105_000]; // Immediate 5% increase
        assert_eq!(check_price_fluctuation(&prices, 0, 5, 3, 0), Some(true));
    }

    #[test]
    fn test_fluctuation_after_intermediates() {
        let prices = vec![
            100_000, // 100.000
            101_000, // 101.000
            102_000, // 102.000
            95_000, // 95.000 (5% decrease)
            106_000 // 106.000 (6% increase)
        ];
        assert_eq!(
            check_price_fluctuation(&prices, 0, 5, 3, 0),
            Some(false) // First fluctuation meeting the threshold is a 5% decrease
        );
    }
}
