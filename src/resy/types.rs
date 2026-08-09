use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResyConfig {
    pub api_key: String,
    pub auth_token: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVenuesRequest {
    pub query: String,
    pub per_page: Option<u32>,
    pub types: Vec<String>,
    pub geo: Option<GeoLocation>,
    pub slot_filter: Option<SlotFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotFilter {
    pub day: String,
    pub party_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueSearchResponse {
    pub search: SearchResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub hits: Vec<Venue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub id: VenueId,
    pub name: String,
    pub location: VenueLocation,
    pub rating: Option<f64>,
    pub price_range: Option<u32>,
    pub cuisine: Option<String>,
    pub url_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueId {
    pub resy: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueLocation {
    pub name: String,
    pub neighborhood: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindAvailabilityRequest {
    pub venue_id: u64,
    pub day: String,
    pub party_size: u32,
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub results: AvailabilityResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityResults {
    pub venues: Vec<VenueAvailability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueAvailability {
    pub venue: VenueInfo,
    pub slots: Vec<TimeSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueInfo {
    pub id: VenueId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub config: SlotConfig,
    pub date: SlotDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotConfig {
    #[serde(rename = "type")]
    pub slot_type: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotDate {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingDetailsRequest {
    pub config_id: String,
    pub day: String,
    pub party_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingDetailsResponse {
    pub book_token: BookToken,
    pub payment_methods: Vec<PaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookToken {
    pub value: String,
    pub date_expires: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: u64,
    pub provider_name: String,
    pub last_four: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookReservationRequest {
    pub book_token: String,
    pub struct_payment_method: String,
    pub source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookReservationResponse {
    pub resy_token: String,
    pub confirmation_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPasswordRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteria {
    pub id: String,
    pub restaurant_name: String,
    pub venue_id: Option<u64>,
    pub date: String,
    pub party_size: u32,
    pub preferred_time: String,
    pub time_range_start: Option<String>,
    pub time_range_end: Option<String>,
    pub enabled: bool,
    pub auto_book: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollingResult {
    pub search_id: String,
    pub found_at: DateTime<Utc>,
    pub venue_name: String,
    pub time_slot: String,
    pub booking_token: Option<String>,
    pub booked: bool,
    pub reservation_token: Option<String>,
}
