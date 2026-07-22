"""
E2E validation tests for the Dioxus application.

These tests validate that the core Dioxus app functionality is working:
- Me page loads and displays content (default)
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


def test_me_page_loads_and_displays_content(page: Page):
    """Test that the Me page loads successfully and displays content."""
    # Wait for the page to be fully loaded and hydrated
    page.wait_for_load_state("networkidle")

    # Check that the body is visible (basic page load validation)
    body = page.locator("body")
    expect(body).to_be_visible()

    # Check that the bottom tab bar is present (layout component)
    tab_bar = page.locator("#bottom-tab-bar")
    expect(tab_bar).to_be_visible()

    # Check that Me page placeholder content is visible
    heading = page.get_by_role("heading", name="me")
    expect(heading).to_be_visible()


def test_bottom_tab_bar_navigation_works(page: Page, base_url: str):
    """Test that navigation tabs in the bottom tab bar work correctly."""
    # Wait for initial load
    page.wait_for_load_state("networkidle")

    # Check that all three tabs exist and are visible
    me_tab = page.get_by_role("tab", name="me")
    expect(me_tab).to_be_visible()

    dialogue_tab = page.get_by_role("tab", name="dialogue")
    expect(dialogue_tab).to_be_visible()

    misc_tab = page.get_by_role("tab", name="misc")
    expect(misc_tab).to_be_visible()

    # Click the Dialogue tab and verify navigation
    dialogue_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify URL changed to /dialogue
    expect(page).to_have_url(build_url(base_url, "dialogue"))

    # Click the Misc tab
    misc_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify URL changed to /misc
    expect(page).to_have_url(build_url(base_url, "misc"))

    # Navigate back to Me
    me_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify we're back on Me page
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
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    dialogue_tab.click()
    page.wait_for_load_state("networkidle")

    # Verify the tab bar is still visible after interaction
    expect(tab_bar).to_be_visible()

    # Verify navigation actually worked
    expect(page).to_have_url(build_url(base_url, "dialogue"))


def test_dialogue_route_works(page: Page, base_url: str):
    """Test that the dialogue route works."""
    # Navigate directly to dialogue route
    page.goto(build_url(base_url, "dialogue"))
    page.wait_for_load_state("networkidle")

    # Verify we're on the dialogue page
    expect(page).to_have_url(build_url(base_url, "dialogue"))

    # Verify tab bar is still present (layout should work)
    tab_bar = page.locator("#bottom-tab-bar")
    expect(tab_bar).to_be_visible()

    # Verify Dialogue tab is active
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    expect(dialogue_tab).to_have_attribute("aria-selected", "true")


def test_routing_system_works(page: Page, base_url: str):
    """Test that the Dioxus routing system works end-to-end."""
    # Start on Me (default route)
    page.goto(build_url(base_url))
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url))

    # Navigate to Dialogue via tab
    dialogue_tab = page.get_by_role("tab", name="dialogue")
    dialogue_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "dialogue"))

    # Navigate to Misc via tab
    misc_tab = page.get_by_role("tab", name="misc")
    misc_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url, "misc"))

    # Navigate back to Me via tab
    me_tab = page.get_by_role("tab", name="me")
    me_tab.click()
    page.wait_for_load_state("networkidle")
    expect(page).to_have_url(build_url(base_url))
