"""
E2E validation tests for the Dioxus application.

These tests validate that the core Dioxus app functionality is working:
- Map View page loads and displays content
- Navigation between routes works
- App is interactive and hydrated
- Bottom tab bar navigation is functional

To run these tests:
1. Start the Dioxus server: dx serve --platform web
2. Run tests: uv run pytest tests/test_dioxus_app.py
"""

from urllib.parse import urljoin

from playwright.sync_api import Page, expect


def build_url(base_url: str, path: str = "") -> str:
    """
    Normalize and join the base URL with a relative path.
    Ensures consistent trailing slashes and works when the base URL includes a subpath.
    """
    normalized_base = base_url.rstrip("/") + "/"
    normalized_path = path.lstrip("/")
    return urljoin(normalized_base, normalized_path)


def test_map_view_page_loads_and_displays_content(page: Page):
    """Test that the Map View page loads successfully and displays content."""
    # Wait for the page to be fully loaded and hydrated
    page.wait_for_load_state("networkidle")

    # Check that the body is visible (basic page load validation)
    body = page.locator("body")
    expect(body).to_be_visible()

    # Check that the bottom tab bar is present (layout component)
    tab_bar = page.locator("#bottom-tab-bar")
    expect(tab_bar).to_be_visible()

    # Check that Map View placeholder content is visible
    heading = page.get_by_role("heading", name="Map View")
    expect(heading).to_be_visible()


def test_bottom_tab_bar_navigation_works(page: Page, base_url: str):
    """Test that navigation tabs in the bottom tab bar work correctly."""
    # Wait for initial load
    page.wait_for_load_state("networkidle")

    # Check that all three tabs exist and are visible
    map_tab = page.get_by_role("tab", name="Map View")
    expect(map_tab).to_be_visible()

    account_tab = page.get_by_role("tab", name="Account")
    expect(account_tab).to_be_visible()

    social_tab = page.get_by_role("tab", name="Social Feed")
    expect(social_tab).to_be_visible()

    # Click the Account tab and verify navigation
    account_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify URL changed to /account
    expect(page).to_have_url(build_url(base_url, "account"))

    # Click the Social Feed tab
    social_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify URL changed to /social
    expect(page).to_have_url(build_url(base_url, "social"))

    # Navigate back to Map View
    map_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify we're back on Map View page
    expect(page).to_have_url(build_url(base_url))


def test_app_is_fully_hydrated(page: Page, base_url: str):
    """Test that the Dioxus app has fully hydrated and is interactive."""
    # Wait for the page to load
    page.wait_for_load_state("networkidle")

    # Basic check that the page is interactive
    body = page.locator("body")
    expect(body).to_be_visible()

    # Try interacting with the page to ensure Dioxus hydration worked
    # Click on a tab to ensure events are working
    tab_bar = page.locator("#bottom-tab-bar")
    account_tab = page.get_by_role("tab", name="Account")
    account_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify the tab bar is still visible after interaction
    expect(tab_bar).to_be_visible()

    # Verify navigation actually worked
    expect(page).to_have_url(build_url(base_url, "account"))


def test_account_route_works(page: Page, base_url: str):
    """Test that the account route works."""
    # Navigate directly to account route
    page.goto(build_url(base_url, "account"))
    page.wait_for_load_state("networkidle")

    # Verify we're on the account page
    expect(page).to_have_url(build_url(base_url, "account"))

    # Verify tab bar is still present (layout should work)
    tab_bar = page.locator("#bottom-tab-bar")
    expect(tab_bar).to_be_visible()

    # Verify Account tab is active
    account_tab = page.get_by_role("tab", name="Account")
    expect(account_tab).to_have_attribute("aria-selected", "true")


def test_routing_system_works(page: Page, base_url: str):
    """Test that the Dioxus routing system works end-to-end."""
    # Start on Map View (default route)
    page.goto(build_url(base_url))
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url))

    # Navigate to Account via tab
    account_tab = page.get_by_role("tab", name="Account")
    account_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "account"))

    # Navigate to Social Feed via tab
    social_tab = page.get_by_role("tab", name="Social Feed")
    social_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "social"))

    # Navigate back to Map View via tab
    map_tab = page.get_by_role("tab", name="Map View")
    map_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url))
