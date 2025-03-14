#[cfg(test)]
mod tests {
    use bminsight_lib::core::bmi::*;

    #[test]
    fn test_calculate_bmi_normal() {
        // 5'10", 160 lbs => 23.5
        let bmi_val = calculate_bmi(70, 160);
        assert_eq!(bmi_val, 23.5);

        let category = categorize_bmi(bmi_val);
        assert_eq!(category, BmiCategory::Normal);
    }

    #[test]
    fn test_calculate_bmi_underweight() {
        // 5'10", 120 lbs => 17.6
        let bmi_val = calculate_bmi(70, 120);
        assert_eq!(bmi_val, 17.6);

        let category = categorize_bmi(bmi_val);
        assert_eq!(category, BmiCategory::Underweight);
    }

    #[test]
    fn test_calculate_bmi_obese() {
        // 5'5", 200 lbs => 34.1
        let bmi_val = calculate_bmi(65, 200);
        assert_eq!(bmi_val, 34.1);

        let category = categorize_bmi(bmi_val);
        assert_eq!(category, BmiCategory::Obese);
    }

    #[test]
    fn test_calculate_bmi_overweight() {
        // 5'10", 180 lbs => 25.8
        let bmi_val = calculate_bmi(70, 180);
        assert_eq!(bmi_val, 26.4);

        let category = categorize_bmi(bmi_val);
        assert_eq!(category, BmiCategory::Overweight);
    }
}
