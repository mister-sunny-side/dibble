use crate::components::BottomTabBar;
use crate::MainTab;
use crate::Route;
use dioxus::prelude::*;

const BOTTOM_TAB_LAYOUT_CSS: Asset = asset!("/assets/styling/bottom_tab_layout.css");

/// BottomTabLayout component wraps all main routes with a bottom tab bar navigation.
///
/// This layout component:
/// - Manages the active tab state
/// - Synchronizes tab state with the current route
/// - Renders the bottom tab bar
/// - Renders child routes via Outlet
#[component]
pub fn BottomTabLayout() -> Element {
    let router = router();
    let mut active_tab = use_signal(|| MainTab::MapView);

    // Sync tab state with current route
    use_effect(move || {
        let current_route = router.current::<Route>();
        match current_route {
            Route::MapView { .. } => *active_tab.write() = MainTab::MapView,
            Route::AccountView { .. } => *active_tab.write() = MainTab::Account,
            Route::SocialFeedView { .. } => *active_tab.write() = MainTab::SocialFeed,
        }
    });

    // Handle tab change - navigate to corresponding route
    let handle_tab_change = move |tab: MainTab| {
        let tab_clone = tab.clone();
        *active_tab.write() = tab;
        match tab_clone {
            MainTab::MapView => {
                router.push(Route::MapView {});
            }
            MainTab::Account => {
                router.push(Route::AccountView {});
            }
            MainTab::SocialFeed => {
                router.push(Route::SocialFeedView {});
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: BOTTOM_TAB_LAYOUT_CSS }

        div {
            id: "bottom-tab-layout",
            class: "layout-container",

            // Main content area
            div {
                class: "content-area",
                // The `Outlet` component renders the child route component (MapView, AccountView, or SocialFeedView)
                Outlet::<Route> {}
            }

            // Bottom tab bar
            BottomTabBar {
                active_tab,
                on_tab_change: handle_tab_change,
            }
        }
    }
}
