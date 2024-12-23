use dioxus::prelude::*;
use dioxus_desktop::tao::{self, platform::unix::WindowBuilderExtUnix};
use hyprland::{data::Monitor, shared::HyprDataActive};
use hyprland::dispatch::*;

fn app() -> Element {
    rsx! {
        div {
            "system-panel"
        }
    }
}

fn main() {
    let monitor = Monitor::get_active().expect("No active monitor found");
    let panel_position = tao::dpi::LogicalPosition::new(111, 111);
    let panel_size = tao::dpi::LogicalSize::new(monitor.width, 20);

    let window = tao::window::WindowBuilder::new()
        .with_always_on_top(true)
        .with_decorations(false)
        .with_closable(false)
        .with_skip_taskbar(true)
        .with_title("")
        .with_visible_on_all_workspaces(true)
        .with_min_inner_size(panel_size)
        .with_max_inner_size(panel_size)
        .with_position(panel_position);

    dioxus::LaunchBuilder::new().with_cfg(dioxus::desktop::Config::new().with_window(window)).launch(app);
}
