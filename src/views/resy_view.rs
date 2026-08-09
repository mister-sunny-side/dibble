use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::resy::server::*;
use crate::resy::types::*;

#[component]
pub fn ResyView() -> Element {
    let mut auth_status = use_signal(|| false);
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut search_query = use_signal(|| String::new());
    let mut search_results = use_signal(|| Vec::<Venue>::new());
    let mut search_criteria_list = use_signal(|| Vec::<SearchCriteria>::new());
    let mut polling_results = use_signal(|| Vec::<PollingResult>::new());
    let mut error_message = use_signal(|| Option::<String>::None);

    let authenticate = move |_| {
        spawn(async move {
            #[cfg(feature = "server")]
            {
                match initialize_daemon(email(), password()).await {
                    Ok(_) => {
                        auth_status.set(true);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Authentication failed: {}", e)));
                    }
                }
            }
        });
    };

    let search_restaurants = move |_| {
        spawn(async move {
            #[cfg(feature = "server")]
            {
                match search_venues(search_query(), None, None).await {
                    Ok(venues) => {
                        search_results.set(venues);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Search failed: {}", e)));
                    }
                }
            }
        });
    };

    let load_search_criteria = move |_| {
        spawn(async move {
            #[cfg(feature = "server")]
            {
                match list_search_criteria().await {
                    Ok(criteria) => {
                        search_criteria_list.set(criteria);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to load searches: {}", e)));
                    }
                }
            }
        });
    };

    let load_polling_results = move |_| {
        spawn(async move {
            #[cfg(feature = "server")]
            {
                match list_polling_results(None).await {
                    Ok(results) => {
                        polling_results.set(results);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to load results: {}", e)));
                    }
                }
            }
        });
    };

    rsx! {
        div {
            class: "resy-container",
            h1 { "Resy Reservation Manager" }

            if let Some(ref error) = error_message() {
                div { class: "error-message", "{error}" }
            }

            if !auth_status() {
                div {
                    class: "auth-section",
                    h2 { "Login to Resy" }
                    p { "Enter your Resy credentials to enable automatic reservation polling." }
                    
                    input {
                        r#type: "email",
                        placeholder: "Email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value())
                    }

                    input {
                        r#type: "password",
                        placeholder: "Password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value())
                    }

                    button {
                        onclick: authenticate,
                        "Login & Start Daemon"
                    }
                }
            } else {
                div {
                    class: "main-content",
                    
                    div {
                        class: "search-section",
                        h2 { "Search Restaurants" }
                        
                        input {
                            r#type: "text",
                            placeholder: "Search for a restaurant...",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value())
                        }

                        button {
                            onclick: search_restaurants,
                            "Search"
                        }

                        if !search_results().is_empty() {
                            div {
                                class: "search-results",
                                h3 { "Search Results" }
                                for venue in search_results() {
                                    VenueCard { venue }
                                }
                            }
                        }
                    }

                    div {
                        class: "criteria-section",
                        h2 { "Active Searches" }
                        button {
                            onclick: load_search_criteria,
                            "Refresh"
                        }

                        if !search_criteria_list().is_empty() {
                            for criteria in search_criteria_list() {
                                SearchCriteriaCard { criteria }
                            }
                        } else {
                            p { "No active searches. Add one above!" }
                        }
                    }

                    div {
                        class: "results-section",
                        h2 { "Polling Results" }
                        button {
                            onclick: load_polling_results,
                            "Refresh"
                        }

                        if !polling_results().is_empty() {
                            for result in polling_results() {
                                PollingResultCard { result }
                            }
                        } else {
                            p { "No results yet." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VenueCard(venue: Venue) -> Element {
    rsx! {
        div {
            class: "venue-card",
            h4 { "{venue.name}" }
            p { "{venue.location.name}" }
            if let Some(ref neighborhood) = venue.location.neighborhood {
                p { class: "neighborhood", "{neighborhood}" }
            }
            if let Some(ref cuisine) = venue.cuisine {
                p { class: "cuisine", "{cuisine}" }
            }
            button {
                onclick: move |_| {
                },
                "Add to Watch List"
            }
        }
    }
}

#[component]
fn SearchCriteriaCard(criteria: SearchCriteria) -> Element {
    rsx! {
        div {
            class: "criteria-card",
            h4 { "{criteria.restaurant_name}" }
            p { "Date: {criteria.date}" }
            p { "Party Size: {criteria.party_size}" }
            p { "Preferred Time: {criteria.preferred_time}" }
            p { "Auto-book: {criteria.auto_book}" }
            p { "Enabled: {criteria.enabled}" }
        }
    }
}

#[component]
fn PollingResultCard(result: PollingResult) -> Element {
    rsx! {
        div {
            class: "result-card",
            h4 { "{result.venue_name}" }
            p { "Found at: {result.found_at}" }
            p { "Time Slot: {result.time_slot}" }
            if result.booked {
                p { class: "booked", "✓ Booked!" }
                if let Some(ref token) = result.reservation_token {
                    p { "Confirmation: {token}" }
                }
            } else {
                p { class: "available", "Available but not booked" }
            }
        }
    }
}
