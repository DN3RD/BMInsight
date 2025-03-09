use serde::Serialize;
use tauri::{State, command};

use crate::core::state::AppState;
use crate::core::bmi::{calculate_bmi, categorize_bmi, get_feedback, BmiCategory, BmiInput};

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[derive(Serialize)]
pub struct BmiResult {
    pub bmi_value: f64,
    pub category: BmiCategory,
    pub feedback: String,
}

#[command]
pub async fn compute_bmi(
    state: State<'_, AppState>,
    age: u8,
    gender: String,
    height_inches: u16,
    weight_pounds: u16
) -> Result<BmiResult, String> {
    let val = calculate_bmi(height_inches, weight_pounds);
    let cat = categorize_bmi(val);

    let feedback = get_feedback(
        state.llm.as_ref(),
        &BmiInput {
            age,
            gender,
            bmi_value: val,
            bmi_category: cat.clone(),
        }
    ).await?; 
    
    Ok(BmiResult {
        bmi_value: val,
        category: cat,
        feedback,
    })
}


