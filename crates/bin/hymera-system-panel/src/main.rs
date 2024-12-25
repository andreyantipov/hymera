mod ui;
mod window;

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use hyprland::{data::Monitor, shared::HyprDataActive};
use hyprland::dispatch::*;
use ui::Panel;

fn main() {
    let panel_window: WindowBuilder = window::create();

    LaunchBuilder::new()
        .with_cfg(dioxus::desktop::Config::new()
        .with_window(panel_window))
        .launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            Panel {}
        }
    }
}