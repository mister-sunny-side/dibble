use crate::MainTab;
use dioxus::prelude::*;

const BOTTOM_TAB_BAR_CSS: Asset = asset!("/assets/styling/bottom_tab_bar.css");

/// BottomTabBar component provides navigation between the three main tabs.
///
/// Props:
/// - `active_tab`: Signal containing the currently active tab
/// - `on_tab_change`: Callback function that is called when a tab is clicked
#[component]
pub fn BottomTabBar(active_tab: Signal<MainTab>, on_tab_change: EventHandler<MainTab>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOTTOM_TAB_BAR_CSS }

        div {
            id: "bottom-tab-bar",
            role: "tablist",
            aria_label: "Main navigation",

            // Me tab (left, default)
            button {
                class: if active_tab() == MainTab::Me { "tab-button active" } else { "tab-button" },
                role: "tab",
                aria_selected: active_tab() == MainTab::Me,
                onclick: move |_| on_tab_change.call(MainTab::Me),
                "me"
            }

            // Dialogue tab (center)
            button {
                class: if active_tab() == MainTab::Dialogue { "tab-button active" } else { "tab-button" },
                role: "tab",
                aria_selected: active_tab() == MainTab::Dialogue,
                onclick: move |_| on_tab_change.call(MainTab::Dialogue),
                "dialogue"
            }

            // Misc tab (right)
            button {
                class: if active_tab() == MainTab::Misc { "tab-button active" } else { "tab-button" },
                role: "tab",
                aria_selected: active_tab() == MainTab::Misc,
                onclick: move |_| on_tab_change.call(MainTab::Misc),
                "misc"
            }
        }
    }
}
