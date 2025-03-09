#![allow(non_snake_case)]

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::{Gender, Ruler, Scale};


pub fn App() -> Element {
    let weightSignal = use_signal(|| "1".to_string());
    let ageSignal = use_signal(|| "1".to_string());
    let genderSignal = use_signal(move || "male".to_string());
    let heightSignal = use_signal(|| 48u16);
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