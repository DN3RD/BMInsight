#[cfg(test)]
mod tests {

    use bminsight_lib::core::bmi::*;

    /// # Testing `calculate_bmi(height_inches, weight_pounds)`
    ///
    /// ## Equivalence classes to consider:
    /// - Zero or near-zero height (invalid or edge case -> BMI = 0.0)
    /// - "Typical" valid heights/weights (normal usage)
    /// - Very large height/weight (upper boundary)
    ///
    /// ## Boundary tests:
    /// - height_inches = 0
    /// - weight_pounds = 0
    /// - Some typical boundary combo around "normal" or extremes

    #[test]
    fn test_bmi_zero_height() {
        // BVA for height at 0
        let bmi_val = calculate_bmi(0, 180);
        // The function is expected to return 0.0 if height_m is 0
        assert_eq!(bmi_val, 0.0);
    }

    #[test]
    fn test_bmi_zero_weight() {
        // BVA for weight at 0
        let bmi_val = calculate_bmi(70, 0);
        // weight_kg = 0 => BMI = 0
        assert_eq!(bmi_val, 0.0);
    }

    #[test]
    fn test_bmi_typical_normal() {
        // Normal usage: e.g. 5'10", 160 lb => ~23.5
        let bmi_val = calculate_bmi(70, 160);
        assert_eq!(bmi_val, 23.5);
    }

    #[test]
    fn test_bmi_typical_overweight() {
        // e.g. 5'10", 180 lb => ~26.4
        let bmi_val = calculate_bmi(70, 180);
        assert_eq!(bmi_val, 26.4);
    }

    #[test]
    fn test_bmi_large_values() {
        // e.g. 6'6" (78 inches), 300 lb => ~35.5
        let bmi_val = calculate_bmi(78, 300);
        // Check a plausible result for tall/heavy scenario
        assert_eq!(bmi_val, 35.5);
    }

    /// # Testing `categorize_bmi(bmi)`
    ///
    /// ## BMI Categories (BVA around thresholds):
    /// - Underweight: < 18.5
    /// - Normal:      18.5 .. 25.0
    /// - Overweight:  25.0 .. 30.0
    /// - Obese:       >= 30.0
    ///
    /// Tests are performed just below, exactly at, and just above each boundary:
    /// - 18.5 boundary (Underweight <-> Normal)
    /// - 25.0 boundary (Normal <-> Overweight)
    /// - 30.0 boundary (Overweight <-> Obese)

    #[test]
    fn test_bmi_category_underweight_boundary() {
        // Just below 18.5 => Underweight
        let category = categorize_bmi(18.4);
        assert_eq!(category, BmiCategory::Underweight);

        // Exactly 18.5 => Normal
        let category = categorize_bmi(18.5);
        assert_eq!(category, BmiCategory::Normal);

        // Just above 18.5 => Normal
        let category = categorize_bmi(18.6);
        assert_eq!(category, BmiCategory::Normal);
    }

    #[test]
    fn test_bmi_category_overweight_boundary() {
        // Just below 25.0 => Normal
        let category = categorize_bmi(24.9);
        assert_eq!(category, BmiCategory::Normal);

        // Exactly 25.0 => Overweight
        let category = categorize_bmi(25.0);
        assert_eq!(category, BmiCategory::Overweight);

        // Just above 25.0 => Overweight
        let category = categorize_bmi(25.1);
        assert_eq!(category, BmiCategory::Overweight);
    }

    #[test]
    fn test_bmi_category_obese_boundary() {
        // Just below 30.0 => Overweight
        let category = categorize_bmi(29.9);
        assert_eq!(category, BmiCategory::Overweight);

        // Exactly 30.0 => Obese
        let category = categorize_bmi(30.0);
        assert_eq!(category, BmiCategory::Obese);

        // Just above 30.0 => Obese
        let category = categorize_bmi(30.1);
        assert_eq!(category, BmiCategory::Obese);
    }
}
