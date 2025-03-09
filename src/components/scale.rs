#![allow(non_snake_case)]
use dioxus::prelude::*;
use web_sys::{console};
#[component]
pub fn Scale(max: u32, min: u32, title: String, scaleValue: Signal<String>) -> Element {
    rsx!{
        link { rel: "stylesheet", href: "assets/scale.css" }
        div{
            class: "column scale-wrapper",
            h4 {
                "{title}"
            }
            div {
                class: "row scale",
                div {
                    class: "button",
                    onclick: move |_| {
                        if let Ok(num) = scaleValue().parse::<u32>() {
                            if num > 1 {
                                scaleValue.set(format!("{}",num-min));
                            }
                        } else {
                            scaleValue.set("{min}".parse().unwrap());
                        }
                    },
                    "-"
                }
                input {
                    r#type: "number",
                    class: "input-value",
                    value: scaleValue(),
                    min: "{min}",
                    max: "{max}",
                    oninput: move |evt| {
                        if let Ok(num) = evt.value().parse::<u32>() {
                            if num <= max && num > 0 {
                                scaleValue.set(evt.value());
                            }else if num<=0 {
                                scaleValue.set("{min}".parse().unwrap());
                            }else{
                                scaleValue.set("{max}".parse().unwrap());
                            }
                        } else {
                            scaleValue.set(String::from(""));
                        }}
                }
                div {
                    class: "button",
                    onclick: move |_| {
                        if let Ok(num) = scaleValue().parse::<u32>() {
                            if num < max {
                                scaleValue.set(format!("{}",num+1));
                            }
                        } else {
                            scaleValue.set("{min}".parse().unwrap());
                        }
                    },
                    "+"
                }
            }
        }
    }
}