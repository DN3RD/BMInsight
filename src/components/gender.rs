#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Gender(gender: Signal<String>) -> Element {
    rsx!{
        link { rel: "stylesheet", href: "assets/gender.css" }
        div {
                class: "row gender-wrapper",
                div {
                    class: format!("column gender {}", if gender() == "male" { "highlight-male" } else { "" }),
                    onclick: move |_| gender.set("male".to_string()),
                    span{
                        class: "material-icons gender-icon male",
                        "male"
                    }
                    "Male"
                }
                div {
                    class: format!("column gender {}", if gender() == "female" { "highlight-female" } else { "" }),
                onclick: move |_| gender.set("female".to_string()),
                    span{
                        class: "material-icons md-48 gender-icon female",
                        "female"
                    }
                    "Female"
                }
            }
    }
}