#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PanelProps {
    children: Element
}


#[component]
pub fn Panel(props: PanelProps) -> Element {
    rsx! {
        div { class: "panel",
            {props.children}
        }
    }
}
