use crate::resy::error::{ResyError, ResyResult};
use crate::resy::types::*;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};

/// Mock Resy client for testing without hitting the real API
pub struct MockResyClient {
    config: ResyConfig,
    authenticated: bool,
}

impl MockResyClient {
    pub fn new(config: ResyConfig) -> Self {
        Self {
            config,
            authenticated: false,
        }
    }

    pub async fn authenticate(&mut self) -> ResyResult<String> {
        info!("MOCK: Authenticating with email: {:?}", self.config.email);
        
        // Simulate network delay
        time::sleep(Duration::from_millis(500)).await;

        if self.config.email.as_ref().map(|e| e.contains('@')).unwrap_or(false) {
            self.authenticated = true;
            let mock_token = "mock_auth_token_12345";
            self.config.auth_token = Some(mock_token.to_string());
            info!("MOCK: Authentication successful");
            Ok(mock_token.to_string())
        } else {
            Err(ResyError::AuthenticationError("Invalid email format".to_string()))
        }
    }

    pub async fn search_venues(&self, request: SearchVenuesRequest) -> ResyResult<Vec<Venue>> {
        info!("MOCK: Searching for venues with query: {}", request.query);
        
        // Simulate network delay
        time::sleep(Duration::from_millis(300)).await;

        // Return mock venues based on query
        let venues = vec![
            Venue {
                id: VenueId { resy: 1001 },
                name: format!("{} Downtown", request.query),
                location: VenueLocation {
                    name: "New York, NY".to_string(),
                    neighborhood: Some("Manhattan".to_string()),
                },
                rating: Some(4.5),
                price_range: Some(3),
                cuisine: Some("Italian".to_string()),
                url_slug: format!("{}-downtown", request.query.to_lowercase()),
            },
            Venue {
                id: VenueId { resy: 1002 },
                name: format!("{} Midtown", request.query),
                location: VenueLocation {
                    name: "New York, NY".to_string(),
                    neighborhood: Some("Midtown".to_string()),
                },
                rating: Some(4.3),
                price_range: Some(2),
                cuisine: Some("American".to_string()),
                url_slug: format!("{}-midtown", request.query.to_lowercase()),
            },
            Venue {
                id: VenueId { resy: 1003 },
                name: format!("The {} Experience", request.query),
                location: VenueLocation {
                    name: "Brooklyn, NY".to_string(),
                    neighborhood: Some("Williamsburg".to_string()),
                },
                rating: Some(4.7),
                price_range: Some(4),
                cuisine: Some("French".to_string()),
                url_slug: format!("the-{}-experience", request.query.to_lowercase()),
            },
        ];

        info!("MOCK: Returning {} venues", venues.len());
        Ok(venues)
    }

    pub async fn find_availability(&self, request: FindAvailabilityRequest) -> ResyResult<Vec<TimeSlot>> {
        info!("MOCK: Finding availability for venue {} on {}", request.venue_id, request.day);
        
        // Simulate network delay
        time::sleep(Duration::from_millis(400)).await;

        // Simulate occasional "no availability" responses
        if request.venue_id % 5 == 0 {
            warn!("MOCK: No availability for venue {}", request.venue_id);
            return Ok(Vec::new());
        }

        // Return mock time slots
        let slots = vec![
            TimeSlot {
                config: SlotConfig {
                    slot_type: "Standard".to_string(),
                    token: format!("mock_slot_token_{}_{}_1730", request.venue_id, request.day),
                },
                date: SlotDate {
                    start: format!("{}T17:30:00", request.day),
                    end: format!("{}T19:30:00", request.day),
                },
            },
            TimeSlot {
                config: SlotConfig {
                    slot_type: "Standard".to_string(),
                    token: format!("mock_slot_token_{}_{}_1830", request.venue_id, request.day),
                },
                date: SlotDate {
                    start: format!("{}T18:30:00", request.day),
                    end: format!("{}T20:30:00", request.day),
                },
            },
            TimeSlot {
                config: SlotConfig {
                    slot_type: "Bar".to_string(),
                    token: format!("mock_slot_token_{}_{}_2030", request.venue_id, request.day),
                },
                date: SlotDate {
                    start: format!("{}T20:30:00", request.day),
                    end: format!("{}T22:30:00", request.day),
                },
            },
        ];

        info!("MOCK: Returning {} available slots", slots.len());
        Ok(slots)
    }

    pub async fn get_booking_details(&self, config_id: &str, day: &str, party_size: u32) -> ResyResult<BookingDetailsResponse> {
        info!("MOCK: Getting booking details for config_id: {}", config_id);
        
        // Simulate network delay
        time::sleep(Duration::from_millis(350)).await;

        let response = BookingDetailsResponse {
            book_token: BookToken {
                value: format!("mock_book_token_{}", config_id),
                date_expires: format!("{}T23:59:59", day),
            },
            payment_methods: vec![
                PaymentMethod {
                    id: 9001,
                    provider_name: "Visa".to_string(),
                    last_four: "4242".to_string(),
                },
                PaymentMethod {
                    id: 9002,
                    provider_name: "Mastercard".to_string(),
                    last_four: "5555".to_string(),
                },
            ],
        };

        info!("MOCK: Returning booking details with {} payment methods", response.payment_methods.len());
        Ok(response)
    }

    pub async fn book_reservation(&self, book_token: &str, payment_method_id: u64) -> ResyResult<BookReservationResponse> {
        info!("MOCK: Booking reservation with token: {} and payment method: {}", book_token, payment_method_id);
        
        // Simulate longer booking delay
        time::sleep(Duration::from_millis(800)).await;

        let response = BookReservationResponse {
            resy_token: format!("mock_resy_token_{}", chrono::Utc::now().timestamp()),
            confirmation_number: format!("MOCK-{}", chrono::Utc::now().timestamp() % 100000),
        };

        info!("MOCK: Successfully 'booked' reservation: {}", response.confirmation_number);
        Ok(response)
    }
}

/// Helper function to create a mock client from config
pub fn create_mock_client(config: ResyConfig) -> MockResyClient {
    MockResyClient::new(config)
}
