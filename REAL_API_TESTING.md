# Testing with the Real Resy API

This guide walks you through testing the Resy integration with actual API calls.

## Prerequisites

1. **Resy Account** (recommended: use a separate test account, not your primary one)
2. **Development Environment** with:
   - Rust 1.83+ installed
   - Dioxus CLI (`dx`) installed
   - All dependencies resolved

## Setup Instructions

### 1. Start the Dioxus Server

```bash
cd /workspace
dx serve --platform web
```

Wait for the build to complete. You should see:
```
Local: http://localhost:8080
```

### 2. Open the Application

Navigate to `http://localhost:8080` in your browser.

### 3. Navigate to the Resy Tab

Click on the "Resy" tab in the bottom navigation bar.

## Testing the Real API

### Authentication

1. **Skip the Test Mode section** (the green box)
2. Scroll to the "**🔴 Real Mode**" section (red warning box)
3. Enter your Resy credentials:
   - **Email**: Your Resy account email
   - **Password**: Your Resy account password
4. Click "**Login with Real Account & Start Daemon**"

**What happens:**
- The system makes a POST request to `https://api.resy.com/3/auth/password`
- On success, you get an auth token
- The background polling daemon starts automatically
- You'll see the UI switch to the authenticated state

### Search for Restaurants

1. In the search box, enter a restaurant name or cuisine type
   - Examples: "Carbone", "Italian", "Sushi"
2. Click "Search Restaurants"

**What happens:**
- Makes a GET request to `https://api.resy.com/4/find`
- Returns real restaurants from Resy's database
- Each result shows:
  - Restaurant name
  - Location & neighborhood
  - Rating (if available)
  - Price range ($-$$$$)
  - Cuisine type

### Add Search Criteria

For any restaurant in the search results:

1. Click "**Add Search**" button
2. Fill in the criteria form:
   - **Date**: When you want to dine (format: YYYY-MM-DD)
   - **Party Size**: Number of people (1-20)
   - **Preferred Time**: Your ideal time (format: HH:MM, e.g., "19:00" for 7PM)
   - **Time Range** (optional):
     - Start: Earliest acceptable time
     - End: Latest acceptable time
   - **Auto-book**: Toggle ON if you want automatic booking

3. Click "**Save**"

**What happens:**
- Criteria is saved to the `sled` database (`./data/resy.db/`)
- The background daemon will start checking this restaurant every 5 minutes
- If auto-book is enabled, it will automatically book when a match is found

### Monitor Polling Results

The "**Polling Results**" section automatically updates with:

- **Status**: `found`, `no_availability`, or `booked`
- **Timestamp**: When the check was performed
- **Restaurant & Date**: What was checked
- **Details**: 
  - If `found`: Which time slots are available
  - If `booked`: Confirmation number from Resy
  - If `no_availability`: Just logs the attempt

### Background Daemon Behavior

Once authenticated, the daemon:

1. **Runs every 5 minutes** automatically
2. **Checks all enabled search criteria** in the database
3. **For each criterion:**
   - Calls `GET /4/find` with availability filters
   - Parses available time slots
   - Matches against your time preferences
   - If auto-book enabled and match found → attempts booking

4. **Booking Flow** (when auto-book triggers):
   - Gets booking details: `GET /3/details`
   - Selects first payment method on file
   - Books reservation: `POST /3/book`
   - Saves confirmation to database

## Rate Limiting

The client is configured with **1 request per second** maximum to avoid triggering Resy's rate limits:

```rust
// From src/resy/client.rs
let quota = Quota::per_second(NonZeroU32::new(1).unwrap());
```

With 5-minute polling intervals, this means:
- Each restaurant = ~3-4 API calls per check
- With 10 active searches = ~40 calls per 5 minutes = ~8 calls/minute
- Well within safe limits

## Testing Scenarios

### Scenario 1: Check Availability (No Auto-Book)

1. Search for a popular restaurant
2. Add search criteria with auto-book **OFF**
3. Set date to tomorrow or next week
4. Wait 5 minutes
5. Check "Polling Results" to see what's available

**Expected**: Results showing available or unavailable time slots, no bookings made.

### Scenario 2: Auto-Book a Reservation

⚠️ **Warning**: This will make a REAL reservation and may charge your payment method on file!

1. Search for a less popular restaurant (better availability)
2. Add search criteria with auto-book **ON**
3. Set reasonable criteria (e.g., party of 2, flexible time range)
4. Wait for daemon to find availability
5. Check "Polling Results" for booking confirmation

**Expected**: When availability matches, you'll see:
- Status: `booked`
- Confirmation number: `RESY-XXXXX`
- You'll receive email confirmation from Resy

### Scenario 3: Multiple Searches

1. Add 3-5 different restaurants with varying criteria
2. Some with auto-book ON, some OFF
3. Watch the polling results accumulate over time

**Expected**: Each restaurant checked independently every 5 minutes.

## Troubleshooting

### "Authentication failed"
- **Check credentials**: Make sure email/password are correct
- **Try web login first**: Verify your account works on resy.com
- **Check API key**: The default key in code is `VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5`

### "Search failed" or empty results
- **Try different queries**: Some restaurants have specific name formats
- **Check spelling**: Restaurant names must match Resy's database
- **Try location**: Add city name (e.g., "Carbone New York")

### No polling results appearing
- **Wait full 5 minutes**: First poll happens immediately, then every 5 minutes
- **Check daemon is running**: It auto-starts on login
- **Verify criteria is saved**: Should appear in "Active Searches" section

### Reservation auto-booked but you didn't want it
- ⚠️ **Cancel immediately** on Resy website or app
- **Disable auto-book** on that search criterion
- **Delete the criterion** if no longer needed

## API Endpoints Used

Here are the actual Resy API endpoints the client hits:

### Authentication
```
POST https://api.resy.com/3/auth/password
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password"
}
```

### Search Venues
```
GET https://api.resy.com/4/find?query=carbone&per_page=20
x-resy-auth-token: {token}
x-resy-api-key: VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5
```

### Find Availability
```
GET https://api.resy.com/4/find?lat=0&long=0&day=2026-08-15&party_size=2&venue_id=12345
x-resy-auth-token: {token}
x-resy-api-key: VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5
```

### Get Booking Details
```
GET https://api.resy.com/3/details?config_id={slot_token}&day=2026-08-15&party_size=2
x-resy-auth-token: {token}
x-resy-api-key: VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5
```

### Book Reservation
```
POST https://api.resy.com/3/book
Content-Type: application/json
x-resy-auth-token: {token}
x-resy-api-key: VbWk7s3L4KiK5fzlO7JD3Q5EYolJI7n5

{
  "book_token": "{token_from_details}",
  "struct_payment_method": "{\"id\": 123}"
}
```

## Database Inspection

The `sled` database stores all data locally:

### Location
- Real mode: `./data/resy.db/`
- Test mode: `./data/resy_mock.db/`

### Tables
- `search_criteria`: Your saved restaurant searches
- `polling_results`: Historical results from daemon checks

### Reset Database
```bash
rm -rf ./data/resy.db/
```

## Safety & Best Practices

1. **Use a test account** if possible - Don't risk your primary Resy account
2. **Start with auto-book OFF** - Verify availability checking works first
3. **Test during off-hours** - Less load on Resy's API
4. **Monitor closely when auto-book is ON** - You might book multiple reservations
5. **Cancel unwanted bookings immediately** - Don't be a no-show

## Known Limitations

- **No official API**: This uses reverse-engineered endpoints that could change
- **Rate limiting**: Conservative limits mean slower checking, but safer
- **Payment method**: Uses first payment method on file, can't choose
- **No cancellation**: The system only books, doesn't cancel (do that manually)
- **US only**: Resy is primarily US-based, limited international support

## Need Help?

If you encounter issues:

1. Check the browser console for errors (F12 → Console tab)
2. Look at server logs where `dx serve` is running
3. Verify your Resy account works normally on the website
4. Try the test mode first to verify the UI and flow

## Success Indicators

You'll know it's working when:

✅ Authentication succeeds and UI changes to authenticated state  
✅ Search returns real restaurant names and details  
✅ Adding criteria shows up in "Active Searches"  
✅ Polling results appear after 5 minutes  
✅ (If auto-book enabled) Confirmation number appears in results  
✅ Email from Resy arrives with reservation details  

Happy reservation hunting! 🍽️
