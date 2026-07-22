use dioxus::prelude::*;

/// SocialFeedView component - placeholder page for the Social Feed tab.
///
/// This will eventually display friends' reviews and recommendations.
#[component]
pub fn SocialFeedView() -> Element {
    rsx! {
        div {
            class: "placeholder-page",
            h1 {
                "Social Feed"
            }
            p {
                "Friends' reviews and recommendations will appear here"
            }
        }
    }
}
