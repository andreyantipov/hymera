mod ui;
mod window;

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use hyprland::{data::Monitor, shared::HyprDataActive};
use hyprland::dispatch::*;
use ui::Panel;

fn main() {
    let panel_window: WindowBuilder = window::create();
    let cfg = dioxus::desktop::Config::new();

    LaunchBuilder::new()
        .with_cfg(cfg.with_window(panel_window))
        .launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            Panel {}
        }
    }
}