"""
E2E tests for Navigation Foundation specification.

These tests validate the bottom tab bar navigation system:
- Bottom tab bar renders and is fixed at bottom
- All three tabs (me, dialogue, misc) are present and clickable
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

    # Check for me tab
    me_tab = page.get_by_role("tab", name="me")
    expect(me_tab).to_be_visible()

    # Check for dialogue tab
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    expect(dialogue_tab).to_be_visible()

    # Check for misc tab
    misc_tab = page.get_by_role("tab", name="misc")
    expect(misc_tab).to_be_visible()


def test_tab_navigation_updates_url(page: Page, base_url: str):
    """Test that clicking tabs navigates to the correct routes and updates URL."""
    page.wait_for_load_state("networkidle")

    # Start on me (default route)
    expect(page).to_have_url(build_url(base_url))

    # Click dialogue tab
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    dialogue_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "dialogue"))

    # Click misc tab
    misc_tab = page.get_by_role("tab", name="misc")
    misc_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "misc"))

    # Click me tab
    me_tab = page.get_by_role("tab", name="me")
    me_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url))


def test_active_tab_indicator(page: Page):
    """Test that the active tab has a visual indicator and updates correctly."""
    page.wait_for_load_state("networkidle")

    # Initially me should be active
    me_tab = page.get_by_role("tab", name="me")
    expect(me_tab).to_have_attribute("aria-selected", "true")
    expect(me_tab).to_have_class(re.compile("active"))

    # Click dialogue tab
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    dialogue_tab.click()
    page.wait_for_load_state("networkidle")

    # Dialogue should now be active
    expect(dialogue_tab).to_have_attribute("aria-selected", "true")
    expect(dialogue_tab).to_have_class(re.compile("active"))

    # me should no longer be active
    expect(me_tab).to_have_attribute("aria-selected", "false")
    expect(me_tab).not_to_have_class(re.compile("active"))

    # Click misc tab
    misc_tab = page.get_by_role("tab", name="misc")
    misc_tab.click()
    page.wait_for_load_state("networkidle")

    # Misc should now be active
    expect(misc_tab).to_have_attribute("aria-selected", "true")
    expect(misc_tab).to_have_class(re.compile("active"))

    # Dialogue should no longer be active
    expect(dialogue_tab).to_have_attribute("aria-selected", "false")
    expect(dialogue_tab).not_to_have_class(re.compile("active"))


def test_deep_linking_map_view(page: Page, base_url: str):
    """Test that navigating directly to / sets me tab as active."""
    page.goto(build_url(base_url))
    page.wait_for_load_state("networkidle")

    expect(page).to_have_url(build_url(base_url))

    me_tab = page.get_by_role("tab", name="me")
    expect(me_tab).to_have_attribute("aria-selected", "true")
    expect(me_tab).to_have_class(re.compile("active"))


def test_deep_linking_account(page: Page, base_url: str):
    """Test that navigating directly to /dialogue sets dialogue tab as active."""
    page.goto(build_url(base_url, "dialogue"))
    page.wait_for_load_state("networkidle")

    expect(page).to_have_url(build_url(base_url, "dialogue"))

    dialogue_tab = page.get_by_role("tab", name="dialogue")
    expect(dialogue_tab).to_have_attribute("aria-selected", "true")
    expect(dialogue_tab).to_have_class(re.compile("active"))


def test_deep_linking_social_feed(page: Page, base_url: str):
    """Test that navigating directly to /misc sets misc tab as active."""
    page.goto(build_url(base_url, "misc"))
    page.wait_for_load_state("networkidle")

    expect(page).to_have_url(build_url(base_url, "misc"))

    misc_tab = page.get_by_role("tab", name="misc")
    expect(misc_tab).to_have_attribute("aria-selected", "true")
    expect(misc_tab).to_have_class(re.compile("active"))


def test_map_view_placeholder_displays(page: Page):
    """Test that me page placeholder content is visible."""
    page.wait_for_load_state("networkidle")

    # Check for placeholder heading
    heading = page.get_by_role("heading", name="me")
    expect(heading).to_be_visible()

    # Check for placeholder description
    description = page.get_by_text("Personal content will appear here")
    expect(description).to_be_visible()


def test_account_view_placeholder_displays(page: Page, base_url: str):
    """Test that dialogue page placeholder content is visible."""
    page.goto(build_url(base_url, "dialogue"))
    page.wait_for_load_state("networkidle")

    # Check for placeholder heading
    heading = page.get_by_role("heading", name="dialogue")
    expect(heading).to_be_visible()

    # Check for placeholder description
    description = page.get_by_text("Blog posts will appear here")
    expect(description).to_be_visible()


def test_social_feed_view_placeholder_displays(page: Page, base_url: str):
    """Test that misc page placeholder content is visible."""
    page.goto(build_url(base_url, "misc"))
    page.wait_for_load_state("networkidle")

    # Check for placeholder heading
    heading = page.get_by_role("heading", name="misc")
    expect(heading).to_be_visible()

    # Check for placeholder description
    description = page.get_by_text("Miscellaneous content will appear here")
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
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    dialogue_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify still only one active tab
    active_count = count_active_tabs()
    assert active_count == 1, f"Expected exactly 1 active tab after click, but found {active_count}"
