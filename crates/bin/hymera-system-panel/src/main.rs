mod ui;
mod window;

use dioxus::prelude::*;
use ui::dummy_clock::DummyClock;
use ui::panel::Panel;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const PANEL_CSS: Asset = asset!("/assets/styling/panel.css");

fn main() {
    let panel_window = window::create();
    let config = dioxus::desktop::Config::new();

    LaunchBuilder::new()
        .with_cfg(config.with_window(panel_window))
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: PANEL_CSS }
        Panel {
           "hymera-system-panel"
           DummyClock {  }
        }
    }
}
