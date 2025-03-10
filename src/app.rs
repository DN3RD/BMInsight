#![allow(non_snake_case)]

use crate::components::{Gender, Ruler, Scale};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct BmiArgs {
    age: u8,
    gender: String,
    #[serde(rename = "heightInches")]
    height_inches: u16,
    #[serde(rename = "weightPounds")]
    weight_pounds: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BmiFeedback {
    pub interpretation: String,
    pub suggestions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BmiResult {
    bmi_value: f64,
    category: String,
    feedback: BmiFeedback,
}

impl std::fmt::Display for BmiResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BMI: {:.1}, Category: {}, Feedback: {:?}",
            self.bmi_value, self.category, self.feedback
        )
    }
}

pub fn App() -> Element {
    let weightSignal = use_signal(|| "1".to_string());
    let ageSignal = use_signal(|| "1".to_string());
    let genderSignal = use_signal(move || "male".to_string());
    let heightSignal = use_signal(|| 48u16);

    let mut hiddenSignal = use_signal(move || true);

    let mut bmi_result = use_signal(|| BmiResult {
        bmi_value: 0.0,
        category: String::new(),
        feedback: BmiFeedback {
            interpretation: String::new(),
            suggestions: vec![],
        },
    });

    let getBmi = move |_: MouseEvent| async move {
        let bmi_args = serde_wasm_bindgen::to_value(&BmiArgs {
            age: ageSignal().parse().unwrap(),
            gender: genderSignal(),
            height_inches: heightSignal(),
            weight_pounds: weightSignal().parse().unwrap(),
        })
        .unwrap();

        let bmi = invoke("compute_bmi", bmi_args).await;
        bmi_result.set(serde_wasm_bindgen::from_value(bmi).unwrap());
        hiddenSignal.set(false)
    };
    let bmi = bmi_result().clone();
    rsx! {
        link { rel: "stylesheet", href: "assets/styles.css" }
        link { rel: "stylesheet", href:"https://fonts.googleapis.com/icon?family=Material+Icons" }
        main {
            class: "main",
            div {
                class: "container",
                div {
                    class: "column form-container",
                    h2 {
                        class: "title",
                        "BMI Insight"
                    }
                    Gender {
                        gender: genderSignal
                    }
                    Ruler {
                        height: heightSignal,
                    }
                    Scale {
                        max: 1000,
                        min: 1,
                        title: "Weight (in Lb)",
                        scaleValue: weightSignal,

                    }
                    Scale {
                        max: 100,
                        min: 1,
                        title: "Age",
                        scaleValue: ageSignal,
                    }
                }
            }
            div {
                class: format!("result-container-wrapper {}", if hiddenSignal() {""} else {"full-page"}),
                onclick: move |_| {
                    hiddenSignal.set(true);
                },
                div {
                    class: "result-container",
                    div {
                        class: "circle",
                        onclick: getBmi,
                        "BMI"
                    }
                    div {
                        class: format!("result-wrapper {}", if hiddenSignal() {"hidden"} else {""}),
                        p {
                            class: "result-header",
                            "Your BMI is"
                        }
                        p {
                            class: "result-value",
                            "{bmi.bmi_value} Kg/m"
                            sup { "2" }
                        }
                        p {
                            class: "result-category",
                            "{bmi_result().category}"
                        }

                        p {
                             class: "interpretation-text",
                            "{bmi_result().feedback.interpretation}"
                        }
                        
                        ul {
                            style: "list-style-type: disc; text-align: left; margin-left: 1em;",
                            for suggestion in &bmi_result().feedback.suggestions {
                                li { "{suggestion}" }
                            }
                        }

                    }
                }
            }
        }
    }
}
