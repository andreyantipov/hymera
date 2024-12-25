#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Panel() -> Element {
    rsx! {
        p {
            b { "Dioxus Labs" }
            " An Open Source project dedicated to making Rust UI wonderful."
        }
        div { "style": "width: 20px; height: 20px; background-color: red;" }
    }
}

