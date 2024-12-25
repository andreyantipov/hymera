use dioxus_desktop::{LogicalSize, WindowBuilder};
use dioxus_desktop::tao::{platform::unix::WindowBuilderExtUnix};
use hyprland::{data::Monitor, shared::HyprDataActive};

fn setup() {
    const SYSTEM_PANEL_WORKSPACE_GAP_RULE: &str = ",addreserved, 20, 0, 0, 0";
    const SYSTEM_PANEL_POSITION_RULE: &str = "move 0 0, class:^(hymera-system-panel)$";
    const SYSTEM_PANEL_STYLE_RULE: &str = "nofocus, norounding, class:^(hymera-system-panel)$";

    hyprland::keyword::Keyword::set("monitor", SYSTEM_PANEL_WORKSPACE_GAP_RULE).expect("Could not configure panel gap");
    hyprland::keyword::Keyword::set("windowrulev2", SYSTEM_PANEL_POSITION_RULE).expect("Could not configure panel position");
    hyprland::keyword::Keyword::set("windowrulev2", SYSTEM_PANEL_STYLE_RULE).expect("Could not configure panel style");
}

pub fn create() -> WindowBuilder {
    setup();

    let monitor = Monitor::get_active().expect("No active monitor found");
    let panel_size = LogicalSize::new(monitor.width as f32 / monitor.scale, 30.0);

    WindowBuilder::new()
        .with_always_on_top(true)
        .with_decorations(false)
        .with_closable(false)
        .with_skip_taskbar(true)
        .with_title("")
        .with_visible_on_all_workspaces(true)
        .with_inner_size(panel_size)
        .with_min_inner_size(panel_size)
        .with_max_inner_size(panel_size)
}