#![allow(non_snake_case)]

use dioxus::html::geometry::euclid::num::Ceil;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window, HtmlElement, ScrollToOptions};

fn format_height(height: u16) -> String {
    let feet = height / 12;
    let inches = height % 12;
    if inches == 0 {
        format!("{}'", feet)
    } else {
        format!("{}'{}\"", feet, inches)
    }
}

// New Ruler component
#[component]
pub fn Ruler(height: Signal<u16>) -> Element {
    let mut scroll_position = use_signal(|| (height()-48u16) as f64);
    let onscroll = move |event: Event<ScrollData>| {
        if let Some(target) = event.data.as_web_event().target().and_then(|target| target.dyn_into::<HtmlElement>().ok()){
            let target_scroll = target.scroll_left() as f64;
            let scroll_index = ( target_scroll - 16f64)/32f64;
            let index =  scroll_index.ceil();
            if(target_scroll <= index*32f64 + 16f64 && target_scroll >= index*32f64 - 16f64 ) {
                scroll_position.set(index);
            } else {
                scroll_position.set(scroll_index);
            }
        }
    };
    use_effect(move || {
        height.set((scroll_position() + 48f64) as u16);
    });
    rsx! {
        link { rel: "stylesheet", href: "assets/ruler.css" }
        div
        {
            class: "ruler-wrapper",
            h4 {
                "Height (in ft/in)"
            }
            div{
                class: "row ruler",
                id: "ruler",
                onscroll: onscroll,
                div{
                    class: "row ruler-container",
                    for (index, height) in (48..=96).enumerate() {
                        div {
                            class: if index as f64 == scroll_position() { "tick-container highlight" } else { "tick-container" },
                            span {
                                class: if height % 6 == 0 { "major-unit" } else { "minor-unit" },
                                "{format_height(height)}"
                            }
                            div { class: if height % 6 == 0 { "major-tick" } else { "minor-tick" } }
                        }
                    }
                }
            }
        }
    }
}
