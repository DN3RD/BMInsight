#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::components::{Gender, Ruler, Scale};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct BmiArgs {
    age: u8,
    gender: String,
    heightInches: u16,
    weightPounds: u16,
}

#[derive(Serialize, Deserialize)]
struct BmiResult {
    bmi_value: f64,
    category: String,
    feedback: String
}

impl std::fmt::Display for BmiResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BMI: {:.1}, Category: {}, Feedback: {}", self.bmi_value, self.category, self.feedback)
    }
}

pub fn App() -> Element {
    let weightSignal = use_signal(|| "1".to_string());
    let ageSignal = use_signal(|| "1".to_string());
    let genderSignal = use_signal(move || "male".to_string());
    let heightSignal = use_signal(|| 48u16);
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());

    let mut bmi_result = use_signal(|| {
        BmiResult {
            bmi_value: 0.0,
            category: String::new(),
            feedback: String::new(),
        }
    });


    let greet = move |_: FormEvent| async move {
        if name.read().is_empty() {
            return;
        }

        let name = name.read();
        let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
        let bmi_args = serde_wasm_bindgen::to_value(&BmiArgs {
            age: 22,
            gender: "male".to_string(),
            heightInches: 70,
            weightPounds: 180,
        }).unwrap();

        let bmi = invoke("compute_bmi", bmi_args).await;

        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        let new_msg = invoke("greet", args).await.as_string().unwrap();
        greet_msg.set(new_msg);
        bmi_result.set(serde_wasm_bindgen::from_value(bmi).unwrap());
    };

    rsx! {
        link { rel: "stylesheet", href: "assets/styles.css" }
        link { rel: "stylesheet", href:"https://fonts.googleapis.com/icon?family=Material+Icons" }
        main {
            class: "main",
            div {
                class: "column container",
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
            div {
                class: "result-container-wrapper",
                div {
                    class: "result-container",
                    div {
                        class: "circle",
                        "BMI"
                    }
                    div {
                        class: "result-wrapper hidden",
                        p {
                            class: "result-header",
                            "Your BMI is"
                        }
                        p {
                            class: "result-value",
                            "19.6 Kg/m"
                            sup { "2" }
                        }
                        p {
                            class: "result-category",
                            "(Normal)"
                        }
                        p {
                            class: "result-recommendation",
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus lacinia odio vitae vestibulum. Sed sollicitudin erat eu augue interdum, in convallis libero feugiat. Morbi ac nulla sed felis iaculis luctus et at purus. Ut ut bibendum elit. Mauris iaculis, mauris et auctor consequat, nisl erat cursus metus, at iaculis felis lectus a erat. Proin ut libero et enim efficitur auctor. Sed et lacinia orci. Curabitur a odio mi."
                        }
                    }
                }
            }
        }
    }
}