use dioxus::prelude::*;

/// AccountView component - placeholder page for the Account tab.
///
/// This will eventually display user profile and settings.
#[component]
pub fn AccountView() -> Element {
    rsx! {
        div {
            class: "placeholder-page",
            h1 {
                "Account"
            }
            p {
                "Profile and settings will appear here"
            }
        }
    }
}
