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
    }
}