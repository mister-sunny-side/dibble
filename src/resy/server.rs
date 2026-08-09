use crate::resy::client::ResyClient;
use crate::resy::daemon::{ResyClientType, ResyDaemon};
use crate::resy::mock_client::MockResyClient;
use crate::resy::types::*;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref DAEMON: Arc<Mutex<Option<ResyDaemon>>> = Arc::new(Mutex::new(None));
}

#[server(SearchVenues)]
pub async fn search_venues(
    query: String,
    date: Option<String>,
    party_size: Option<u32>,
) -> Result<Vec<Venue>, ServerFnError> {
    let config = ResyConfig::default();
    let client = ResyClient::new(config).map_err(|e| ServerFnError::new(e.to_string()))?;

    let request = SearchVenuesRequest {
        query,
        per_page: Some(20),
        types: vec!["venue".to_string()],
        geo: None,
        slot_filter: if let (Some(d), Some(ps)) = (date, party_size) {
            Some(SlotFilter {
                day: d,
                party_size: ps,
            })
        } else {
            None
        },
    };

    let venues = client
        .search_venues(request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(venues)
}

#[server(CheckAvailability)]
pub async fn check_availability(
    venue_id: u64,
    date: String,
    party_size: u32,
) -> Result<Vec<TimeSlot>, ServerFnError> {
    let config = ResyConfig::default();
    let client = ResyClient::new(config).map_err(|e| ServerFnError::new(e.to_string()))?;

    let request = FindAvailabilityRequest {
        venue_id,
        day: date,
        party_size,
        lat: 0.0,
        long: 0.0,
    };

    let slots = client
        .find_availability(request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(slots)
}

#[server(SaveSearchCriteria)]
pub async fn save_search_criteria(criteria: SearchCriteria) -> Result<(), ServerFnError> {
    let daemon = DAEMON.lock().unwrap();
    
    if let Some(ref daemon) = *daemon {
        daemon
            .save_search_criteria(&criteria)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    } else {
        Err(ServerFnError::new("Daemon not initialized".to_string()))
    }
}

#[server(DeleteSearchCriteria)]
pub async fn delete_search_criteria(id: String) -> Result<(), ServerFnError> {
    let daemon = DAEMON.lock().unwrap();
    
    if let Some(ref daemon) = *daemon {
        daemon
            .delete_search_criteria(&id)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    } else {
        Err(ServerFnError::new("Daemon not initialized".to_string()))
    }
}

#[server(ListSearchCriteria)]
pub async fn list_search_criteria() -> Result<Vec<SearchCriteria>, ServerFnError> {
    let daemon = DAEMON.lock().unwrap();
    
    if let Some(ref daemon) = *daemon {
        let criteria = daemon
            .list_search_criteria()
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(criteria)
    } else {
        Err(ServerFnError::new("Daemon not initialized".to_string()))
    }
}

#[server(ListPollingResults)]
pub async fn list_polling_results(
    search_id: Option<String>,
) -> Result<Vec<PollingResult>, ServerFnError> {
    let daemon = DAEMON.lock().unwrap();
    
    if let Some(ref daemon) = *daemon {
        let results = daemon
            .list_polling_results(search_id)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(results)
    } else {
        Err(ServerFnError::new("Daemon not initialized".to_string()))
    }
}

#[server(InitializeDaemon)]
pub async fn initialize_daemon(
    email: String,
    password: String,
) -> Result<String, ServerFnError> {
    let mut config = ResyConfig::default();
    config.email = Some(email);
    config.password = Some(password);

    let mut client = ResyClient::new(config.clone()).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let token = client
        .authenticate()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    config.auth_token = Some(token.clone());
    
    let daemon = ResyDaemon::new_with_real(client, "./data/resy.db")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    daemon
        .start()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut daemon_lock = DAEMON.lock().unwrap();
    *daemon_lock = Some(daemon);

    Ok(token)
}

#[server(InitializeMockDaemon)]
pub async fn initialize_mock_daemon(
    email: String,
) -> Result<String, ServerFnError> {
    let mut config = ResyConfig::default();
    config.email = Some(email);
    config.password = Some("mock_password".to_string());

    let daemon = ResyDaemon::new_with_mock(config, "./data/resy_mock.db")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    daemon
        .start()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut daemon_lock = DAEMON.lock().unwrap();
    *daemon_lock = Some(daemon);

    Ok("mock_auth_token_12345".to_string())
}

#[server(SearchVenuesMock)]
pub async fn search_venues_mock(
    query: String,
) -> Result<Vec<Venue>, ServerFnError> {
    let config = ResyConfig::default();
    let client = MockResyClient::new(config);

    let request = SearchVenuesRequest {
        query,
        per_page: Some(20),
        types: vec!["venue".to_string()],
        geo: None,
        slot_filter: None,
    };

    let venues = client
        .search_venues(request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(venues)
}

#[server(StopDaemon)]
pub async fn stop_daemon() -> Result<(), ServerFnError> {
    let daemon = DAEMON.lock().unwrap();
    
    if let Some(ref daemon) = *daemon {
        daemon
            .stop()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    } else {
        Err(ServerFnError::new("Daemon not running".to_string()))
    }
}
