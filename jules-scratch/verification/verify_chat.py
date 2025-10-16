from playwright.sync_api import sync_playwright, expect

def run(playwright):
    browser = playwright.chromium.launch(headless=True)
    context = browser.new_context()
    page = context.new_page()

    try:
        page.goto("http://localhost:1420/chat", timeout=90000, wait_until="networkidle")

        # Wait for the page to load and the select component to be ready
        expect(page.locator('//div[contains(text(), "Select a model")]')).to_be_visible(timeout=30000)

        # Click to open the model selector
        page.click('//div[contains(text(), "Select a model")]')

        # Wait for the options to appear and click the first one
        # Using a more robust selector to find the first model option
        first_option = page.locator('div[role="option"]').first
        expect(first_option).to_be_visible(timeout=10000)
        first_option.click()

        # Type a message
        page.fill('textarea[placeholder="Type your message..."]', "Hello, world!")

        # Click the send button
        page.click('button[type="submit"]')

        # Wait for the response to appear
        expect(page.locator('div.prose').nth(1)).to_be_visible(timeout=30000)

        # Take a screenshot
        page.screenshot(path="jules-scratch/verification/chat_page.png")

    except Exception as e:
        print(f"An error occurred: {e}")
        page.screenshot(path="jules-scratch/verification/chat_page_error.png")

    finally:
        browser.close()

with sync_playwright() as playwright:
    run(playwright)