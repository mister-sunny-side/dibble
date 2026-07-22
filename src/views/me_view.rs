use dioxus::prelude::*;

/// MeView component - default home page for the "me" tab.
///
/// This displays personal information and content about the owner.
#[component]
pub fn MeView() -> Element {
    rsx! {
        div {
            class: "placeholder-page",
            h1 {
                "me"
            }
            p {
                "Personal content will appear here"
            }
        }
    }
}
