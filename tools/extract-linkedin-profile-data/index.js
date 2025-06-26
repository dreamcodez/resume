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

async function dumpPage(page, name) {
  await page.waitForSelector("body", { timeout: 120000 });
  await page.waitForTimeout(3000);
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
  await page.waitForTimeout(3000);
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
      if (el.className) {
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
}

(async () => {
  const browser = await chromium.launch({ headless: false, slowMo: 2000 });
  const context = await browser.newContext({
    userAgent:
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
  });
  const page = await context.newPage();
  try {
    for (const { name, url } of PAGES) {
      console.log(`\n🌐 Navigating to ${url}`);
      await page.goto(url);
      if (name === "profile") {
        // Wait for user login on the first page
        console.log("🔐 Please log in to LinkedIn manually if prompted...");
        await page.waitForTimeout(10000);
      }
      await dumpPage(page, name);
    }
    console.log("\n✅ All pages dumped.");
  } catch (e) {
    console.error("❌ Error during data dump:", e);
  } finally {
    await browser.close();
  }
})();
