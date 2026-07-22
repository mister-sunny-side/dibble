"""
E2E tests for Navigation Foundation (0.1.0) specification.

These tests validate the bottom tab bar navigation system:
- Bottom tab bar renders and is fixed at bottom
- All three tabs are present and clickable
- Tab navigation updates URL correctly
- Active tab indicator updates correctly
- Deep linking works for all three routes
- Placeholder pages display correct content

To run these tests:
1. Start the Dioxus server: dx serve --platform web
2. Run tests: uv run pytest tests/test_navigation_0_1_0.py
"""

import re
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


def test_bottom_tab_bar_renders(page: Page):
    """Test that the bottom tab bar is visible and fixed at bottom."""
    # Wait for the page to be fully loaded and hydrated
    page.wait_for_load_state("networkidle")

    # Check that the bottom tab bar is present
    tab_bar = page.locator("#bottom-tab-bar")
    expect(tab_bar).to_be_visible()

    # Verify it's fixed at bottom (check computed styles)
    position = tab_bar.evaluate("el => window.getComputedStyle(el).position")
    assert position == "fixed", "Tab bar should be fixed position"

    bottom = tab_bar.evaluate("el => window.getComputedStyle(el).bottom")
    assert bottom == "0px", "Tab bar should be at bottom of viewport"


def test_all_three_tabs_are_present(page: Page):
    """Test that all three tabs are present and visible."""
    page.wait_for_load_state("networkidle")

    # Check for Social Feed tab
    social_tab = page.get_by_role("tab", name="Social Feed")
    expect(social_tab).to_be_visible()

    # Check for Map View tab
    map_tab = page.get_by_role("tab", name="Map View")
    expect(map_tab).to_be_visible()

    # Check for Account tab
    account_tab = page.get_by_role("tab", name="Account")
    expect(account_tab).to_be_visible()


def test_tab_navigation_updates_url(page: Page, base_url: str):
    """Test that clicking tabs navigates to the correct routes and updates URL."""
    page.wait_for_load_state("networkidle")

    # Start on Map View (default route)
    expect(page).to_have_url(build_url(base_url))

    # Click Social Feed tab
    social_tab = page.get_by_role("tab", name="Social Feed")
    social_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "social"))

    # Click Account tab
    account_tab = page.get_by_role("tab", name="Account")
    account_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "account"))

    # Click Map View tab
    map_tab = page.get_by_role("tab", name="Map View")
    map_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url))


def test_active_tab_indicator(page: Page):
    """Test that the active tab has a visual indicator and updates correctly."""
    page.wait_for_load_state("networkidle")

    # Initially Map View should be active
    map_tab = page.get_by_role("tab", name="Map View")
    expect(map_tab).to_have_attribute("aria-selected", "true")
    expect(map_tab).to_have_class(re.compile("active"))

    # Click Social Feed tab
    social_tab = page.get_by_role("tab", name="Social Feed")
    social_tab.click()
    page.wait_for_load_state("networkidle")

    # Social Feed should now be active
    expect(social_tab).to_have_attribute("aria-selected", "true")
    expect(social_tab).to_have_class(re.compile("active"))

    # Map View should no longer be active
    expect(map_tab).to_have_attribute("aria-selected", "false")
    expect(map_tab).not_to_have_class(re.compile("active"))

    # Click Account tab
    account_tab = page.get_by_role("tab", name="Account")
    account_tab.click()
    page.wait_for_load_state("networkidle")

    # Account should now be active
    expect(account_tab).to_have_attribute("aria-selected", "true")
    expect(account_tab).to_have_class(re.compile("active"))

    # Social Feed should no longer be active
    expect(social_tab).to_have_attribute("aria-selected", "false")
    expect(social_tab).not_to_have_class(re.compile("active"))


def test_deep_linking_map_view(page: Page, base_url: str):
    """Test that navigating directly to / sets Map View tab as active."""
    page.goto(build_url(base_url))
    page.wait_for_load_state("networkidle")

    expect(page).to_have_url(build_url(base_url))

    map_tab = page.get_by_role("tab", name="Map View")
    expect(map_tab).to_have_attribute("aria-selected", "true")
    expect(map_tab).to_have_class(re.compile("active"))


def test_deep_linking_account(page: Page, base_url: str):
    """Test that navigating directly to /account sets Account tab as active."""
    page.goto(build_url(base_url, "account"))
    page.wait_for_load_state("networkidle")

    expect(page).to_have_url(build_url(base_url, "account"))

    account_tab = page.get_by_role("tab", name="Account")
    expect(account_tab).to_have_attribute("aria-selected", "true")
    expect(account_tab).to_have_class(re.compile("active"))


def test_deep_linking_social_feed(page: Page, base_url: str):
    """Test that navigating directly to /social sets Social Feed tab as active."""
    page.goto(build_url(base_url, "social"))
    page.wait_for_load_state("networkidle")

    expect(page).to_have_url(build_url(base_url, "social"))

    social_tab = page.get_by_role("tab", name="Social Feed")
    expect(social_tab).to_have_attribute("aria-selected", "true")
    expect(social_tab).to_have_class(re.compile("active"))


def test_map_view_placeholder_displays(page: Page):
    """Test that Map View placeholder content is visible."""
    page.wait_for_load_state("networkidle")

    # Check for placeholder heading
    heading = page.get_by_role("heading", name="Map View")
    expect(heading).to_be_visible()

    # Check for placeholder description
    description = page.get_by_text("Restaurant map will appear here")
    expect(description).to_be_visible()


def test_account_view_placeholder_displays(page: Page, base_url: str):
    """Test that Account View placeholder content is visible."""
    page.goto(build_url(base_url, "account"))
    page.wait_for_load_state("networkidle")

    # Check for placeholder heading
    heading = page.get_by_role("heading", name="Account")
    expect(heading).to_be_visible()

    # Check for placeholder description
    description = page.get_by_text("Profile and settings will appear here")
    expect(description).to_be_visible()


def test_social_feed_view_placeholder_displays(page: Page, base_url: str):
    """Test that Social Feed View placeholder content is visible."""
    page.goto(build_url(base_url, "social"))
    page.wait_for_load_state("networkidle")

    # Check for placeholder heading
    heading = page.get_by_role("heading", name="Social Feed")
    expect(heading).to_be_visible()

    # Check for placeholder description
    description = page.get_by_text("Friends' reviews and recommendations will appear here")
    expect(description).to_be_visible()


def test_only_one_tab_active_at_a_time(page: Page):
    """Test that only one tab is active at a time."""
    page.wait_for_load_state("networkidle")

    # Get all tabs
    tabs = page.locator('[role="tab"]')

    # Count tabs that have the active class by checking each tab's class attribute
    def count_active_tabs():
        count = 0
        for i in range(tabs.count()):
            tab = tabs.nth(i)
            class_attr = tab.get_attribute("class") or ""
            if "active" in class_attr:
                count += 1
        return count

    active_count = count_active_tabs()
    assert active_count == 1, f"Expected exactly 1 active tab, but found {active_count}"

    # Click another tab
    social_tab = page.get_by_role("tab", name="Social Feed")
    social_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify still only one active tab
    active_count = count_active_tabs()
    assert active_count == 1, f"Expected exactly 1 active tab after click, but found {active_count}"
