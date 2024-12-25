#![allow(non_snake_case)]

use dioxus::prelude::*;


pub fn Panel() -> Element {
    let mut count = use_signal(|| 0);
    let text = use_signal(|| "...".to_string());

    rsx! {
        p {
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            " An Open Source project dedicated to making Rust UI wonderful."
            "Server said: {text}"
        }
    }
    }
