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

            // Social Feed tab (left)
            button {
                class: if active_tab() == MainTab::SocialFeed { "tab-button active" } else { "tab-button" },
                role: "tab",
                aria_selected: active_tab() == MainTab::SocialFeed,
                onclick: move |_| on_tab_change.call(MainTab::SocialFeed),
                "Social Feed"
            }

            // Map View tab (center)
            button {
                class: if active_tab() == MainTab::MapView { "tab-button active" } else { "tab-button" },
                role: "tab",
                aria_selected: active_tab() == MainTab::MapView,
                onclick: move |_| on_tab_change.call(MainTab::MapView),
                "Map View"
            }

            // Account tab (right)
            button {
                class: if active_tab() == MainTab::Account { "tab-button active" } else { "tab-button" },
                role: "tab",
                aria_selected: active_tab() == MainTab::Account,
                onclick: move |_| on_tab_change.call(MainTab::Account),
                "Account"
            }
        }
    }
}
