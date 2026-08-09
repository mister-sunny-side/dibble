use crate::resy::error::{ResyError, ResyResult};
use crate::resy::types::*;
use governor::{Quota, RateLimiter};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

const RESY_API_BASE: &str = "https://api.resy.com";
const DEFAULT_API_KEY: &str = "VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5";

pub struct ResyClient {
    client: Client,
    config: ResyConfig,
    rate_limiter: Arc<RateLimiter<governor::state::direct::NotKeyed, governor::clock::DefaultClock>>,
}

impl ResyClient {
    pub fn new(config: ResyConfig) -> ResyResult<Self> {
        let quota = Quota::per_minute(NonZeroU32::new(10).unwrap());
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ResyError::ConfigError(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            client,
            config,
            rate_limiter,
        })
    }

    async fn check_rate_limit(&self) -> ResyResult<()> {
        self.rate_limiter.check().map_err(|_| {
            warn!("Rate limit exceeded, waiting...");
            ResyError::RateLimitExceeded
        })
    }

    fn build_headers(&self) -> ResyResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        headers.insert(
            "X-Resy-API-Key",
            HeaderValue::from_str(&self.config.api_key)
                .map_err(|e| ResyError::ConfigError(format!("Invalid API key: {}", e)))?
        );

        if let Some(auth_token) = &self.config.auth_token {
            headers.insert(
                "X-Resy-Auth-Token",
                HeaderValue::from_str(auth_token)
                    .map_err(|e| ResyError::ConfigError(format!("Invalid auth token: {}", e)))?
            );
            headers.insert(
                "X-Resy-Universal-Auth",
                HeaderValue::from_str(auth_token)
                    .map_err(|e| ResyError::ConfigError(format!("Invalid auth token: {}", e)))?
            );
        }

        headers.insert(
            "User-Agent",
            HeaderValue::from_static("Dibble/1.0")
        );

        Ok(headers)
    }

    pub async fn authenticate(&mut self) -> ResyResult<String> {
        self.check_rate_limit().await?;

        let email = self.config.email.as_ref()
            .ok_or_else(|| ResyError::AuthenticationError("Email not provided".to_string()))?;
        
        let password = self.config.password.as_ref()
            .ok_or_else(|| ResyError::AuthenticationError("Password not provided".to_string()))?;

        info!("Authenticating with Resy API");

        let request = AuthPasswordRequest {
            email: email.clone(),
            password: password.clone(),
        };

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Resy-API-Key",
            HeaderValue::from_str(&self.config.api_key).unwrap()
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = self.client
            .post(format!("{}/3/auth/password", RESY_API_BASE))
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("No body"));
            error!("Authentication failed: {} - {}", status, body);
            return Err(ResyError::AuthenticationError(format!("Status {}: {}", status, body)));
        }

        let auth_response: AuthResponse = response.json().await?;
        self.config.auth_token = Some(auth_response.token.clone());
        
        info!("Successfully authenticated with Resy");
        Ok(auth_response.token)
    }

    pub async fn search_venues(&self, request: SearchVenuesRequest) -> ResyResult<Vec<Venue>> {
        self.check_rate_limit().await?;

        debug!("Searching venues with query: {}", request.query);

        let headers = self.build_headers()?;

        let response = self.client
            .post(format!("{}/3/venuesearch/search", RESY_API_BASE))
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("No body"));
            error!("Search venues failed: {} - {}", status, body);
            return Err(ResyError::ApiError(format!("Status {}: {}", status, body)));
        }

        let search_response: VenueSearchResponse = response.json().await?;
        Ok(search_response.search.hits)
    }

    pub async fn find_availability(&self, request: FindAvailabilityRequest) -> ResyResult<Vec<TimeSlot>> {
        self.check_rate_limit().await?;

        debug!("Finding availability for venue {} on {}", request.venue_id, request.day);

        let headers = self.build_headers()?;

        let url = format!(
            "{}/4/find?venue_id={}&day={}&party_size={}&lat={}&long={}",
            RESY_API_BASE,
            request.venue_id,
            request.day,
            request.party_size,
            request.lat,
            request.long
        );

        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("No body"));
            
            if status.as_u16() == 410 {
                return Err(ResyError::SlotNotAvailable);
            }
            
            error!("Find availability failed: {} - {}", status, body);
            return Err(ResyError::ApiError(format!("Status {}: {}", status, body)));
        }

        let availability_response: AvailabilityResponse = response.json().await?;
        
        if availability_response.results.venues.is_empty() {
            return Ok(Vec::new());
        }

        Ok(availability_response.results.venues[0].slots.clone())
    }

    pub async fn get_booking_details(&self, config_id: &str, day: &str, party_size: u32) -> ResyResult<BookingDetailsResponse> {
        self.check_rate_limit().await?;

        debug!("Getting booking details for config_id: {}", config_id);

        let headers = self.build_headers()?;

        let request = BookingDetailsRequest {
            config_id: config_id.to_string(),
            day: day.to_string(),
            party_size,
        };

        let response = self.client
            .post(format!("{}/3/details", RESY_API_BASE))
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("No body"));
            
            if status.as_u16() == 410 {
                return Err(ResyError::SlotTaken);
            }
            
            error!("Get booking details failed: {} - {}", status, body);
            return Err(ResyError::ApiError(format!("Status {}: {}", status, body)));
        }

        let details: BookingDetailsResponse = response.json().await?;
        Ok(details)
    }

    pub async fn book_reservation(&self, book_token: &str, payment_method_id: u64) -> ResyResult<BookReservationResponse> {
        self.check_rate_limit().await?;

        info!("Booking reservation with token: {}", book_token);

        let headers = self.build_headers()?;

        let payment_method_json = serde_json::json!({
            "id": payment_method_id
        });

        let form_data = vec![
            ("book_token", book_token.to_string()),
            ("struct_payment_method", payment_method_json.to_string()),
            ("source_id", "resy.com-venue-details".to_string()),
        ];

        let response = self.client
            .post(format!("{}/3/book", RESY_API_BASE))
            .headers(headers)
            .form(&form_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("No body"));
            
            if status.as_u16() == 410 {
                return Err(ResyError::SlotTaken);
            }
            
            error!("Book reservation failed: {} - {}", status, body);
            return Err(ResyError::ApiError(format!("Status {}: {}", status, body)));
        }

        let booking: BookReservationResponse = response.json().await?;
        info!("Successfully booked reservation: {}", booking.confirmation_number);
        
        Ok(booking)
    }
}

impl Default for ResyConfig {
    fn default() -> Self {
        Self {
            api_key: DEFAULT_API_KEY.to_string(),
            auth_token: None,
            email: None,
            password: None,
        }
    }
}
