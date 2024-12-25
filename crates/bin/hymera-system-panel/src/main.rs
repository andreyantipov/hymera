use dioxus::prelude::*;
use dioxus_desktop::tao::{self, platform::unix::WindowBuilderExtUnix};
use dioxus_desktop::tao::monitor::VideoMode;
use dioxus_desktop::tao::window::Fullscreen;
use dioxus_desktop::tao::dpi::LogicalSize;
use hyprland::{data::Monitor, shared::HyprDataActive};
use hyprland::dispatch::*;

const SYSTEM_PANEL_WORKSPACE_GAP_RULE: &str = ",addreserved, 20, 0, 0, 0";
const SYSTEM_PANEL_POSITION_RULE: &str = "move 0 0, class:^(hymera-system-panel)$";
const SYSTEM_PANEL_STYLE_RULE: &str = "nofocus, norounding, class:^(hymera-system-panel)$";

fn main() {
    hyprland::keyword::Keyword::set("monitor", SYSTEM_PANEL_WORKSPACE_GAP_RULE).expect("Could not configure panel gap");
    hyprland::keyword::Keyword::set("windowrulev2", SYSTEM_PANEL_POSITION_RULE).expect("Could not configure panel position");
    hyprland::keyword::Keyword::set("windowrulev2", SYSTEM_PANEL_STYLE_RULE).expect("Could not configure panel style");

    let monitor = Monitor::get_active().expect("No active monitor found");
    let panel_size = LogicalSize::new(monitor.width as f32 / monitor.scale, 30.0);

    let window = tao::window::WindowBuilder::new()
        .with_always_on_top(true)
        .with_decorations(false)
        .with_closable(false)
        .with_skip_taskbar(true)
        .with_title("")
        .with_visible_on_all_workspaces(true)
        .with_inner_size(panel_size)
        .with_min_inner_size(panel_size)
        .with_max_inner_size(panel_size);

    LaunchBuilder::new()
        .with_cfg(dioxus::desktop::Config::new()
        .with_window(window))
        .launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            "hymera-system-panel",
        }
    }
}