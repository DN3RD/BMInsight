#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use web_sys::{console, HtmlElement, ScrollIntoViewOptions, ScrollToOptions};
use wasm_bindgen::prelude::*;

fn format_height(height: u16) -> String {
    let feet = height / 12;
    let inches = height % 12;

    if inches == 0 {
        format!("{}'", feet)
    } else {
        format!("{}'{}\"", feet, inches)
    }
}

pub fn App() -> Element {
    let mut scroll_position = use_signal(|| 0f64);
    let onscroll = move |event: Event<ScrollData>| {
        let data = event.data();
        if let Some(target) = event.data.as_web_event().target().and_then(|target| target.dyn_into::<HtmlElement>().ok()){
            let scroll_index = target.scroll_left() as f64;
            let options = ScrollToOptions::new();
            let index: f64 = ((scroll_index - 12f64)/24f64).ceil();
            options.set_behavior(web_sys::ScrollBehavior::Smooth);
            options.set_left(index *24f64);
            console::log_1(&format!("Scroll event scrollIndex: {:?}", scroll_index).into());
            scroll_position.set(index);
        }
    };
    rsx! {
        link { rel: "stylesheet", href: "assets/styles.css" }
        main {
            class: "row container",
            div {
                class: "row ruler",
                onscroll: onscroll,
                div {
                    class: "row ruler-container",
                    onscroll: onscroll,
                    for (index,height) in (60..=96).enumerate() {
                        div {
                            class: if (index as f64 == scroll_position()) {"tick-container highlight"} else {"tick-container"},
                            span {
                                class: if height % 12 == 0 { "major-unit" } else { "minor-unit" },
                                "{format_height(height)}"
                            } // Interpolate the height in the p element
                            div { class: if height % 12 == 0 { "major-tick" } else { "minor-tick" } }
                        }
                    }
                }
            }
        }
    }
}