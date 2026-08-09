use crate::resy::client::ResyClient;
use crate::resy::mock_client::MockResyClient;
use crate::resy::error::{ResyError, ResyResult};
use crate::resy::types::*;
use chrono::{DateTime, Utc};
use sled::{Db, IVec};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

const SEARCH_CRITERIA_TREE: &str = "search_criteria";
const POLLING_RESULTS_TREE: &str = "polling_results";
const POLL_INTERVAL_MINUTES: u64 = 5;

pub enum ResyClientType {
    Real(ResyClient),
    Mock(MockResyClient),
}

impl ResyClientType {
    async fn authenticate(&mut self) -> ResyResult<String> {
        match self {
            ResyClientType::Real(client) => client.authenticate().await,
            ResyClientType::Mock(client) => client.authenticate().await,
        }
    }

    async fn search_venues(&self, request: SearchVenuesRequest) -> ResyResult<Vec<Venue>> {
        match self {
            ResyClientType::Real(client) => client.search_venues(request).await,
            ResyClientType::Mock(client) => client.search_venues(request).await,
        }
    }

    async fn find_availability(&self, request: FindAvailabilityRequest) -> ResyResult<Vec<TimeSlot>> {
        match self {
            ResyClientType::Real(client) => client.find_availability(request).await,
            ResyClientType::Mock(client) => client.find_availability(request).await,
        }
    }

    async fn get_booking_details(&self, config_id: &str, day: &str, party_size: u32) -> ResyResult<BookingDetailsResponse> {
        match self {
            ResyClientType::Real(client) => client.get_booking_details(config_id, day, party_size).await,
            ResyClientType::Mock(client) => client.get_booking_details(config_id, day, party_size).await,
        }
    }

    async fn book_reservation(&self, book_token: &str, payment_method_id: u64) -> ResyResult<BookReservationResponse> {
        match self {
            ResyClientType::Real(client) => client.book_reservation(book_token, payment_method_id).await,
            ResyClientType::Mock(client) => client.book_reservation(book_token, payment_method_id).await,
        }
    }
}

pub struct ResyDaemon {
    client: Arc<ResyClientType>,
    db: Db,
    running: Arc<tokio::sync::RwLock<bool>>,
}

impl ResyDaemon {
    pub fn new(client: ResyClientType, db_path: &str) -> ResyResult<Self> {
        let db = sled::open(db_path)
            .map_err(|e| ResyError::DatabaseError(format!("Failed to open database: {}", e)))?;

        Ok(Self {
            client: Arc::new(client),
            db,
            running: Arc::new(tokio::sync::RwLock::new(false)),
        })
    }

    pub fn new_with_mock(config: ResyConfig, db_path: &str) -> ResyResult<Self> {
        let mock_client = MockResyClient::new(config);
        Self::new(ResyClientType::Mock(mock_client), db_path)
    }

    pub fn new_with_real(client: ResyClient, db_path: &str) -> ResyResult<Self> {
        Self::new(ResyClientType::Real(client), db_path)
    }

    pub async fn start(&self) -> ResyResult<()> {
        {
            let mut running = self.running.write().await;
            if *running {
                return Err(ResyError::ConfigError("Daemon is already running".to_string()));
            }
            *running = true;
        }

        info!("Starting Resy reservation daemon with {}-minute polling interval", POLL_INTERVAL_MINUTES);

        let client = Arc::clone(&self.client);
        let db = self.db.clone();
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60 * POLL_INTERVAL_MINUTES));
            
            loop {
                interval.tick().await;

                let is_running = *running.read().await;
                if !is_running {
                    info!("Daemon stopped");
                    break;
                }

                if let Err(e) = Self::poll_reservations(&client, &db).await {
                    error!("Error during polling cycle: {}", e);
                }
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> ResyResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        info!("Stopping Resy reservation daemon");
        Ok(())
    }

    async fn poll_reservations(client: &ResyClientType, db: &Db) -> ResyResult<()> {
        info!("Starting polling cycle");

        let search_criteria = Self::load_active_searches(db)?;
        
        if search_criteria.is_empty() {
            debug!("No active search criteria found");
            return Ok(());
        }

        info!("Processing {} active search criteria", search_criteria.len());

        for criteria in search_criteria {
            if let Err(e) = Self::process_search(&client, db, &criteria).await {
                error!("Error processing search {}: {}", criteria.id, e);
            }

            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        info!("Polling cycle completed");
        Ok(())
    }

    async fn process_search(
        client: &ResyClientType,
        db: &Db,
        criteria: &SearchCriteria,
    ) -> ResyResult<()> {
        debug!("Processing search: {} - {}", criteria.id, criteria.restaurant_name);

        let venue_id = if let Some(id) = criteria.venue_id {
            id
        } else {
            let search_request = SearchVenuesRequest {
                query: criteria.restaurant_name.clone(),
                per_page: Some(10),
                types: vec!["venue".to_string()],
                geo: None,
                slot_filter: Some(SlotFilter {
                    day: criteria.date.clone(),
                    party_size: criteria.party_size,
                }),
            };

            let venues = client.search_venues(search_request).await?;
            
            if venues.is_empty() {
                warn!("No venues found for search: {}", criteria.restaurant_name);
                return Ok(());
            }

            let venue = &venues[0];
            info!("Found venue: {} (ID: {})", venue.name, venue.id.resy);
            venue.id.resy
        };

        let availability_request = FindAvailabilityRequest {
            venue_id,
            day: criteria.date.clone(),
            party_size: criteria.party_size,
            lat: 0.0,
            long: 0.0,
        };

        let slots = match client.find_availability(availability_request).await {
            Ok(slots) => slots,
            Err(ResyError::SlotNotAvailable) => {
                debug!("No availability for search: {}", criteria.id);
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        if slots.is_empty() {
            debug!("No slots available for search: {}", criteria.id);
            return Ok(());
        }

        let matching_slot = Self::find_matching_slot(&slots, criteria);

        if let Some(slot) = matching_slot {
            info!("Found matching slot: {} for search: {}", slot.date.start, criteria.id);

            if criteria.auto_book {
                if let Err(e) = Self::attempt_booking(client, db, criteria, &slot).await {
                    error!("Failed to book reservation: {}", e);
                    Self::save_result(db, criteria, &slot, false, None)?;
                }
            } else {
                info!("Auto-book disabled, saving slot for user review");
                Self::save_result(db, criteria, &slot, false, None)?;
            }
        }

        Ok(())
    }

    fn find_matching_slot(slots: &[TimeSlot], criteria: &SearchCriteria) -> Option<TimeSlot> {
        for slot in slots {
            let slot_time = &slot.date.start;
            
            if let Some(ref start) = criteria.time_range_start {
                if slot_time < start {
                    continue;
                }
            }

            if let Some(ref end) = criteria.time_range_end {
                if slot_time > end {
                    continue;
                }
            }

            return Some(slot.clone());
        }

        None
    }

    async fn attempt_booking(
        client: &ResyClientType,
        db: &Db,
        criteria: &SearchCriteria,
        slot: &TimeSlot,
    ) -> ResyResult<()> {
        info!("Attempting to book reservation for search: {}", criteria.id);

        let details = client
            .get_booking_details(&slot.config.token, &criteria.date, criteria.party_size)
            .await?;

        if details.payment_methods.is_empty() {
            return Err(ResyError::ApiError("No payment methods available".to_string()));
        }

        let payment_method = &details.payment_methods[0];
        
        let booking = client
            .book_reservation(&details.book_token.value, payment_method.id)
            .await?;

        info!("Successfully booked reservation: {}", booking.confirmation_number);

        Self::save_result(
            db,
            criteria,
            slot,
            true,
            Some(booking.resy_token.clone()),
        )?;

        Ok(())
    }

    fn save_result(
        db: &Db,
        criteria: &SearchCriteria,
        slot: &TimeSlot,
        booked: bool,
        reservation_token: Option<String>,
    ) -> ResyResult<()> {
        let result = PollingResult {
            search_id: criteria.id.clone(),
            found_at: Utc::now(),
            venue_name: criteria.restaurant_name.clone(),
            time_slot: slot.date.start.clone(),
            booking_token: Some(slot.config.token.clone()),
            booked,
            reservation_token,
        };

        let tree = db.open_tree(POLLING_RESULTS_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        let key = format!("{}:{}", result.search_id, result.found_at.timestamp());
        let value = serde_json::to_vec(&result)?;

        tree.insert(key.as_bytes(), value)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn save_search_criteria(&self, criteria: &SearchCriteria) -> ResyResult<()> {
        let tree = self.db.open_tree(SEARCH_CRITERIA_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        let value = serde_json::to_vec(criteria)?;

        tree.insert(criteria.id.as_bytes(), value)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn delete_search_criteria(&self, id: &str) -> ResyResult<()> {
        let tree = self.db.open_tree(SEARCH_CRITERIA_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        tree.remove(id.as_bytes())
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn get_search_criteria(&self, id: &str) -> ResyResult<Option<SearchCriteria>> {
        let tree = self.db.open_tree(SEARCH_CRITERIA_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        if let Some(value) = tree.get(id.as_bytes())
            .map_err(|e| ResyError::DatabaseError(e.to_string()))? {
            let criteria: SearchCriteria = serde_json::from_slice(&value)?;
            Ok(Some(criteria))
        } else {
            Ok(None)
        }
    }

    pub fn list_search_criteria(&self) -> ResyResult<Vec<SearchCriteria>> {
        let tree = self.db.open_tree(SEARCH_CRITERIA_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        let mut criteria_list = Vec::new();

        for item in tree.iter() {
            let (_key, value) = item.map_err(|e| ResyError::DatabaseError(e.to_string()))?;
            let criteria: SearchCriteria = serde_json::from_slice(&value)?;
            criteria_list.push(criteria);
        }

        Ok(criteria_list)
    }

    fn load_active_searches(db: &Db) -> ResyResult<Vec<SearchCriteria>> {
        let tree = db.open_tree(SEARCH_CRITERIA_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        let mut criteria_list = Vec::new();

        for item in tree.iter() {
            let (_key, value) = item.map_err(|e| ResyError::DatabaseError(e.to_string()))?;
            let criteria: SearchCriteria = serde_json::from_slice(&value)?;
            
            if criteria.enabled {
                criteria_list.push(criteria);
            }
        }

        Ok(criteria_list)
    }

    pub fn list_polling_results(&self, search_id: Option<String>) -> ResyResult<Vec<PollingResult>> {
        let tree = self.db.open_tree(POLLING_RESULTS_TREE)
            .map_err(|e| ResyError::DatabaseError(e.to_string()))?;

        let mut results = Vec::new();

        for item in tree.iter() {
            let (_key, value) = item.map_err(|e| ResyError::DatabaseError(e.to_string()))?;
            let result: PollingResult = serde_json::from_slice(&value)?;
            
            if let Some(ref sid) = search_id {
                if &result.search_id == sid {
                    results.push(result);
                }
            } else {
                results.push(result);
            }
        }

        results.sort_by(|a, b| b.found_at.cmp(&a.found_at));

        Ok(results)
    }
}
