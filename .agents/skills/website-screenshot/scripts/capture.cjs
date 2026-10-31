const { chromium } = require(process.argv[2]);
const [, , , url, output, port] = process.argv;
const deadline = setTimeout(() => {
    console.error('Screenshot capture exceeded 35 seconds');
    process.exit(1);
}, 35000);
(async () => {
    const browser = await chromium.connectOverCDP(`http://127.0.0.1:${port}`, { timeout: 5000 });
    try {
        const context = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
        const page = await context.newPage();
        const response = await page.goto(url, { waitUntil: 'commit', timeout: 15000 });
        if (!response || !response.ok()) throw new Error(`Website returned HTTP ${response?.status()}`);
        console.log(`HTTP ${response.status()}: ${await page.title()}`);
        await page.locator('body').waitFor({ state: 'visible', timeout: 10000 });
        await page.waitForTimeout(1500);
        await page.screenshot({ path: output, fullPage: true, timeout: 10000, animations: 'disabled' });
        await context.close();
        console.log(`Saved ${output}`);
    } finally {
        await browser.close();
    }
})().then(() => clearTimeout(deadline)).catch(error => {
    console.error(error.message);
    process.exit(1);
});
