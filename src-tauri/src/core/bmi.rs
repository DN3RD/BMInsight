use serde::Serialize;
use crate::integrations::LlmProvider;
use crate::utils::numbers::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum BmiCategory {
    Underweight,
    Normal,
    Overweight,
    Obese,
}



//Calculate BMI based on following method : https://extoxnet.orst.edu/faqs/dietcancer/web2/twohowto.html
pub fn calculate_bmi(height_inches: u16, weight_pounds: u16) -> f64 {
    // Step 1: Convert weight from pounds to kg (using 0.45 as conversion factor)
    let weight_kg = weight_pounds as f64 * 0.45;

    // Step 2: Convert height from height_inches to meters (using 0.025 as conversion factor)
    let height_m = height_inches as f64 * 0.025;
    
    // Step 3 & 4: Calculate BMI by dividing weight by height squared
    if height_m > 0.0 {
        round_float(weight_kg / (height_m * height_m), 1)
    } else {
        0.0
    }
}

pub fn categorize_bmi(bmi: f64) -> BmiCategory {
    match bmi {
        b if b < 18.5 => BmiCategory::Underweight,
        b if b < 25.0 => BmiCategory::Normal,
        b if b < 30.0 => BmiCategory::Overweight,
        _ => BmiCategory::Obese,
    }
}



const PROMPT_CONTEXT: &str = r#"You are a health professional providing feedback to a patient based on a predefined parameters :
- Age
- Gender
- BMI value
- BMI category
Each patient prompt will include these params, your response must be structured as a JSON object with the following fields:
 - interpretation : a string that will explain the patient's BMI category, it should be one sentence long.
 - suggestions : a string array that will provide the patient with some suggestions on how to improve their health based on the bmi results and a patients age and gender,
     each array elements would be a bullet point. Max array elements is 5 and min is 3.
Your response should be deterministic with respect to the input parameters."#;

#[derive(Debug)]
pub struct BmiInput {
    pub age: u8,
    pub gender: String,
    pub bmi_value: f64,
    pub bmi_category: BmiCategory,
}

pub async fn get_feedback(llm: &dyn LlmProvider, bmi_input: &BmiInput) -> Result<String, String> {
    let data = format!(
        "BMI value of {:.1} (category: {:?}) for a {} year old {}",
        bmi_input.bmi_value, 
        bmi_input.bmi_category,
        bmi_input.age,
        bmi_input.gender
    );
    
    llm.generate_response(PROMPT_CONTEXT, &data)
        .await
        .map_err(|e| format!("Error generating feedback: {}", e))
}