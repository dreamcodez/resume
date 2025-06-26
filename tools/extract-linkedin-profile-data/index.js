/**
 * extract-linkedin-profile-data
 *
 * BSD 3-Clause License
 * Copyright (c) 2024, Matthew Elder and contributors
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * ... (BSD license text omitted for brevity, include full text in real file) ...
 *
 * Usage:
 *   node index.js <linkedin-username>
 *   # Example: node index.js matthewelder
 *
 * Or as an npm package:
 *   npm install github:dreamcodez/extract-linkedin-profile-data
 *   npx extract-linkedin-profile-data <linkedin-username>
 *
 * Dumps all profile, experience, skills, and recommendations data for the given LinkedIn username.
 * Output is saved in ./dist/ as linkedin-dump-<page>.json
 */

const { chromium } = require("playwright");
const fs = require("fs").promises;
const path = require("path");

const username = process.argv[2];
if (!username) {
  console.error("Usage: node index.js <linkedin-username>");
  process.exit(1);
}

const DIST = path.join(__dirname, "dist");
const PAGES = [
  { name: "profile", url: `https://www.linkedin.com/in/${username}/` },
  {
    name: "experience",
    url: `https://www.linkedin.com/in/${username}/details/experience/`,
  },
  {
    name: "skills",
    url: `https://www.linkedin.com/in/${username}/details/skills/`,
  },
  {
    name: "recommendations",
    url: `https://www.linkedin.com/in/${username}/details/recommendations/`,
  },
];

// Helper function to check if browser is still available
function isBrowserAvailable(browser) {
  try {
    return browser && !browser.isClosed();
  } catch {
    return false;
  }
}

async function dumpPage(page, name, retries = 3) {
  for (let attempt = 1; attempt <= retries; attempt++) {
    try {
      console.log(`📄 Attempt ${attempt}/${retries} for ${name} page...`);

      // Wait for body with shorter timeout
      await page.waitForSelector("body", { timeout: 30000 });
      await page.waitForTimeout(2000);

      // Scroll to load all content
      console.log("📜 Scrolling to load all content...");
      await page.evaluate(async () => {
        return new Promise((resolve) => {
          let totalHeight = 0;
          const distance = 200;
          const timer = setInterval(() => {
            const scrollHeight = document.body.scrollHeight;
            window.scrollBy(0, distance);
            totalHeight += distance;
            if (totalHeight >= scrollHeight) {
              clearInterval(timer);
              resolve();
            }
          }, 100);
        });
      });
      await page.waitForTimeout(2000);

      console.log("🔍 Extracting page data...");
      const dump = await page.evaluate(() => {
        function safeText(element) {
          return element ? element.textContent?.trim() : null;
        }
        function safeHtml(element) {
          return element ? element.outerHTML : null;
        }
        function getAllAttributes(element) {
          if (!element) return {};
          const attrs = {};
          for (let attr of element.attributes) {
            attrs[attr.name] = attr.value;
          }
          return attrs;
        }
        const allElements = Array.from(document.querySelectorAll("*"));
        const byId = {};
        const byDataSection = {};
        const byAriaLabel = {};
        const byClass = {};
        allElements.forEach((el) => {
          if (el.id)
            byId[el.id] = {
              tag: el.tagName,
              text: safeText(el),
              html: safeHtml(el),
              attrs: getAllAttributes(el),
            };
          if (el.getAttribute("data-section")) {
            const ds = el.getAttribute("data-section");
            if (!byDataSection[ds]) byDataSection[ds] = [];
            byDataSection[ds].push({
              tag: el.tagName,
              text: safeText(el),
              html: safeHtml(el),
              attrs: getAllAttributes(el),
            });
          }
          if (el.getAttribute("aria-label")) {
            const al = el.getAttribute("aria-label");
            if (!byAriaLabel[al]) byAriaLabel[al] = [];
            byAriaLabel[al].push({
              tag: el.tagName,
              text: safeText(el),
              html: safeHtml(el),
              attrs: getAllAttributes(el),
            });
          }
          if (el.className && typeof el.className === "string") {
            el.className.split(" ").forEach((cls) => {
              if (cls) {
                if (!byClass[cls]) byClass[cls] = [];
                byClass[cls].push({
                  tag: el.tagName,
                  text: safeText(el),
                  html: safeHtml(el),
                  attrs: getAllAttributes(el),
                });
              }
            });
          }
        });
        return {
          timestamp: new Date().toISOString(),
          url: window.location.href,
          title: document.title,
          fullText: document.body.textContent,
          byId,
          byDataSection,
          byAriaLabel,
          byClass,
          rawHtml: document.documentElement.outerHTML,
        };
      });

      await fs.mkdir(DIST, { recursive: true });
      await fs.writeFile(
        path.join(DIST, `linkedin-dump-${name}.json`),
        JSON.stringify(dump, null, 2)
      );
      console.log(`💾 Dumped ${name} page to dist/linkedin-dump-${name}.json`);
      return; // Success, exit retry loop
    } catch (error) {
      console.error(`❌ Attempt ${attempt} failed for ${name}:`, error.message);
      if (attempt < retries) {
        console.log(`⏳ Waiting 3 seconds before retry...`);
        await page.waitForTimeout(3000);
      } else {
        throw error; // Re-throw on final attempt
      }
    }
  }
}

(async () => {
  console.log("🚀 Launching browser...");
  const browser = await chromium.launch({
    headless: false,
    slowMo: 1000,
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  console.log("📄 Creating new browser context...");
  const context = await browser.newContext({
    userAgent:
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
  });

  console.log("🌐 Opening new page...");
  const page = await context.newPage();

  // Wait a moment for the browser to fully initialize
  await page.waitForTimeout(2000);

  try {
    for (const { name, url } of PAGES) {
      console.log(`\n🌐 Navigating to ${url}`);

      try {
        await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
        console.log(`✅ Successfully loaded ${url}`);

        // Wait a bit more for dynamic content to load
        await page.waitForTimeout(5000);
      } catch (navError) {
        console.error(`❌ Failed to navigate to ${url}:`, navError.message);
        console.log("🔄 Trying with basic navigation...");

        try {
          await page.goto(url, { timeout: 30000 });
          console.log(`✅ Loaded ${url} with basic navigation`);
          await page.waitForTimeout(5000);
        } catch (basicNavError) {
          console.error(
            `❌ Basic navigation also failed:`,
            basicNavError.message
          );
          continue;
        }
      }

      if (name === "profile") {
        // Give more time for login on the first page
        console.log("🔐 Please log in to LinkedIn manually if prompted...");
        console.log("⏳ Waiting 30 seconds for login...");

        try {
          await page.waitForTimeout(30000);

          // Check if we're still on login page and wait more if needed
          const currentUrl = page.url();
          console.log(`📍 Current URL: ${currentUrl}`);

          if (currentUrl.includes("login") || currentUrl.includes("auth")) {
            console.log(
              "🔐 Still on login page, waiting additional 30 seconds..."
            );
            await page.waitForTimeout(30000);
          }
        } catch (error) {
          console.log("⚠️ Error during login wait:", error.message);
          console.log("💡 Please ensure you're logged in and try again.");
          return;
        }
      }

      await dumpPage(page, name);
    }
    console.log("\n✅ All pages dumped successfully!");
  } catch (e) {
    console.error("❌ Error during data dump:", e);
    console.log(
      "💡 You can try running the script again if login issues occurred."
    );
  } finally {
    console.log("🔒 Closing browser...");
    if (isBrowserAvailable(browser)) {
      await browser.close();
    }
  }
})();
