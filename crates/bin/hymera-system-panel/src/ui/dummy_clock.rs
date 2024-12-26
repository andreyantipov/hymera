#![allow(non_snake_case)]

use dioxus::prelude::*;
use chrono::Local;
use std::time::Duration;

#[component]
pub fn DummyClock() -> Element {
    let mut time = use_signal(|| "00:00");
    // let mut interval = time::interval(Duration::from_secs(2)); // Set the interval to 2 seconds

    let container_style = r#"
        padding: 10px;
        background: red;
        color: white;
        font-size: 24px;
        text-align: center;
        border-radius: 5px;
    "#;
    
    // use_coroutine(move |_| async {
    //     time += 1;
    // });

    rsx! {
        div { style: "{container_style}",
            "{time}"
        }
    }
}