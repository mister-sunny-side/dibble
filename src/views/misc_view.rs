use dioxus::prelude::*;

/// MiscView component - miscellaneous content page.
///
/// Empty placeholder for now.
#[component]
pub fn MiscView() -> Element {
    rsx! {
        div {
            class: "placeholder-page",
            h1 {
                "misc"
            }
            p {
                "Miscellaneous content will appear here"
            }
        }
    }
}
