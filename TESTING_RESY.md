# Testing the Resy Reservation System

This guide explains how to test the Resy reservation polling feature without using your real Resy account.

## Test Mode (Recommended)

Test mode uses a **mock client** that simulates all Resy API responses without making any real API calls. This is the safest way to test the entire system.

### What Test Mode Does

- ✅ **Zero real API calls** - completely safe, no risk of bans
- ✅ **Simulates all Resy responses** with realistic mock data
- ✅ **Tests the entire workflow** including:
  - Authentication
  - Restaurant search
  - Availability checking
  - Booking flow
  - Background polling daemon
  - Database persistence
  - UI interactions
- ✅ **Uses separate database** (`data/resy_mock.db`) so no mixing with real data
- ✅ **Instant responses** - no network delays (except simulated ones for realism)

### How to Use Test Mode

1. **Start the Dioxus server:**
   ```bash
   dx serve --platform web
   ```

2. **Navigate to the Resy tab** in your browser (usually `http://localhost:8080`)

3. **Click "🧪 Start in Test Mode"**
   - You can optionally enter a test email (e.g., `test@example.com`)
   - No password required in test mode
   - The system will initialize with mock credentials

4. **Test the features:**

   **Search for restaurants:**
   - Enter any search query (e.g., "Italian", "Sushi", "Steakhouse")
   - Mock venues will be generated based on your query
   - Each search returns 3 mock restaurants with different characteristics

   **Add search criteria:**
   - Click "Add Search" on any mock restaurant
   - Configure:
     - Date (any future date)
     - Party size
     - Preferred time
     - Time range (optional)
     - Auto-booking toggle
   - The criteria is saved to the mock database

   **View polling results:**
   - The background daemon polls every 5 minutes
   - Check the "Polling Results" section to see:
     - When searches were performed
     - What availability was found
     - Booking attempts (if auto-book is enabled)
   - Mock results will show realistic scenarios:
     - Some restaurants with availability
     - Some without
     - Successful "bookings" (no real reservations made!)

5. **Visual indicators:**
   - A green banner at the top reminds you that you're in test mode
   - All data is clearly labeled as simulated

## Mock Data Details

### Mock Venues
The mock client generates 3 restaurants for each search query:
- `{Query} Downtown` - Manhattan, Italian, 4.5★, $$$
- `{Query} Midtown` - Midtown, American, 4.3★, $$
- `The {Query} Experience` - Williamsburg, French, 4.7★, $$$$

### Mock Availability
- Returns 3 time slots per venue (5:30 PM, 6:30 PM, 8:30 PM)
- Venues with ID divisible by 5 return no availability (to test that scenario)
- Each slot has a unique mock token

### Mock Booking Flow
- Booking details include 2 mock payment methods
- "Bookings" complete successfully with mock confirmation numbers
- Format: `MOCK-{timestamp}`

## Real Mode (Use with Caution)

⚠️ **Only use real mode if you:**
- Have a dedicated Resy test account
- Understand the risks of reverse-engineering their API
- Are willing to accept potential account bans
- Want to test with actual restaurant data

### Setting up Real Mode

1. **Create a separate Resy test account** (recommended)
   - Do NOT use your primary Resy account
   - Sign up at https://resy.com
   - Use a different email address

2. **Use the "Real Mode" section** in the UI
   - Enter your test account credentials
   - Click "Login with Real Account & Start Daemon"

3. **Rate limiting protection:**
   - The system is configured to make at most 1 request per second
   - Polling happens every 5 minutes
   - This should help avoid bans, but is not guaranteed

## Testing Without the UI

You can also test the backend directly using server functions:

```rust
// Example: Testing mock search programmatically
use crate::resy::mock_client::MockResyClient;
use crate::resy::types::*;

#[tokio::test]
async fn test_mock_search() {
    let config = ResyConfig::default();
    let client = MockResyClient::new(config);
    
    let request = SearchVenuesRequest {
        query: "Italian".to_string(),
        per_page: Some(20),
        types: vec!["venue".to_string()],
        geo: None,
        slot_filter: None,
    };
    
    let venues = client.search_venues(request).await.unwrap();
    assert_eq!(venues.len(), 3);
    assert!(venues[0].name.contains("Italian"));
}
```

## Database Inspection

Both modes use `sled` embedded database:

- **Test mode:** `data/resy_mock.db/`
- **Real mode:** `data/resy.db/`

You can inspect the database using the included server functions:

```rust
// List all search criteria
list_search_criteria().await

// List all polling results
list_polling_results().await
```

## Troubleshooting

### "Authentication failed" in test mode
- Make sure you entered a valid email format (must contain `@`)
- Try using `test@example.com`

### No polling results showing up
- Wait at least 5 minutes after adding search criteria
- Check that the daemon is running (should auto-start on authentication)
- The first poll happens immediately, then every 5 minutes

### Test mode data persists between sessions
- This is intentional - test data is saved to `data/resy_mock.db/`
- To reset: delete the `data/resy_mock.db/` directory
- Real and test databases are completely separate

## What's Being Tested

When you use test mode, you're validating:

1. ✅ **Frontend UI components** - All React-like Dioxus components
2. ✅ **Server functions** - RPC calls between frontend and backend
3. ✅ **Authentication flow** - Login and token management
4. ✅ **Search logic** - Query processing and result handling
5. ✅ **Criteria management** - CRUD operations on search criteria
6. ✅ **Database operations** - Sled persistence and retrieval
7. ✅ **Background daemon** - Polling loop and task scheduling
8. ✅ **Availability checking** - Slot matching and time filtering
9. ✅ **Booking workflow** - Multi-step reservation process
10. ✅ **Error handling** - Graceful degradation and error display

The **only thing not tested** is the actual Resy API integration - but the mock client's interface is identical to the real client, so swapping between them is seamless.

## Recommended Testing Workflow

1. **Start with test mode** - Verify the UI and workflow
2. **Add various search criteria** - Different dates, times, party sizes
3. **Wait for polling results** - Observe the daemon in action
4. **Test auto-booking** - Enable it on a search and see mock bookings
5. **Delete and modify criteria** - Test CRUD operations
6. **Only move to real mode** if you absolutely need real restaurant data

## Safety First

Remember: The mock mode gives you **complete confidence** that:
- No real reservations will be made
- No API calls will hit Resy's servers
- Your real account is never touched
- You can test freely without consequences

Use test mode for development, debugging, and demonstration. Only use real mode when you're ready to use the system for actual reservation hunting.
