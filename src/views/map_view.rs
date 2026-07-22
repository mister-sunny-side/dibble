use dioxus::prelude::*;

/// MapView component - placeholder page for the Map View tab.
///
/// This will eventually display a restaurant map with locations.
#[component]
pub fn MapView() -> Element {
    rsx! {
        div {
            class: "placeholder-page",
            h1 {
                "Map View"
            }
            p {
                "Restaurant map will appear here"
            }
        }
    }
}
