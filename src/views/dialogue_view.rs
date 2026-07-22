use dioxus::prelude::*;

/// DialogueView component - displays links to blog posts.
///
/// This will contain a list of blog post links.
#[component]
pub fn DialogueView() -> Element {
    rsx! {
        div {
            class: "placeholder-page",
            h1 {
                "dialogue"
            }
            p {
                "Blog posts will appear here"
            }
        }
    }
}
