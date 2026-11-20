// Audits the built site in a real browser: hydration errors, console noise,
// meta tags after client-side navigation, theme toggle, mobile menu, and
// screenshots for design review.
//
//   node scripts/audit.mjs http://127.0.0.1:8080 .shots
//
// Uses an installed Edge/Chrome via playwright-core (no browser download).

import { chromium } from 'playwright-core';
import fs from 'node:fs';

const base = process.argv[2] || 'http://127.0.0.1:8080';
const outDir = process.argv[3] || './shots';
fs.mkdirSync(outDir, { recursive: true });

const channel = process.env.PW_CHANNEL || 'msedge';
const browser = await chromium.launch({ channel });
const problems = [];

async function newPage(width = 1440, height = 900) {
  const page = await browser.newPage({ viewport: { width, height } });
  page.on('console', (msg) => {
    if (msg.type() === 'error' || msg.type() === 'warning') {
      const text = msg.text();
      if (/favicon|Download the React/i.test(text)) return;
      problems.push(`[console:${msg.type()}] ${page.url()} :: ${text}`);
    }
  });
  page.on('pageerror', (err) => problems.push(`[pageerror] ${page.url()} :: ${err.message}`));
  page.on('requestfailed', (req) => {
    problems.push(`[requestfailed] ${req.url()} :: ${req.failure()?.errorText}`);
  });
  return page;
}

async function waitHydrated(page) {
  try {
    await page.waitForFunction(
      () => document.documentElement.getAttribute('data-hydrated') === 'true',
      null,
      { timeout: 20000 }
    );
  } catch {
    problems.push(`[hydrate] ${page.url()} never reported data-hydrated`);
  }
}

const routes = [
  ['/', 'home'],
  ['/projects/', 'projects'],
  ['/projects/dtact/', 'dtact'],
  ['/projects/ensia/', 'ensia'],
  ['/projects/bincode/', 'bincode'],
  ['/projects/rssn/', 'rssn'],
  ['/projects/cargo-slide/', 'cargo-slide'],
  ['/projects/virtue/', 'virtue'],
  ['/projects/aevia/', 'aevia'],
  ['/research/', 'research'],
  ['/security/', 'security'],
  ['/sponsor/', 'sponsor'],
  ['/blog/', 'blog'],
  ['/blog/hello-apich/', 'post'],
  ['/contact/', 'contact'],
  ['/404.html', 'notfound'],
];

const page = await newPage();
for (const [route, name] of routes) {
  const resp = await page.goto(base + route, { waitUntil: 'networkidle' });
  if (!resp || resp.status() >= 400) problems.push(`[http] ${route} -> ${resp?.status()}`);
  await page.waitForTimeout(350);

  const info = await page.evaluate(() => ({
    title: document.title,
    h1: [...document.querySelectorAll('h1')].map((h) => h.textContent.trim()),
    canonical: document.querySelector('link[rel="canonical"]')?.href,
    desc: document.querySelector('meta[name="description"]')?.content,
    lang: document.documentElement.lang,
    theme: document.documentElement.getAttribute('data-theme'),
    bg: getComputedStyle(document.body).backgroundColor,
    fonts: [...document.fonts].filter((f) => f.status === 'loaded').length,
    jsonld: [...document.querySelectorAll('script[type="application/ld+json"]')].length,
  }));

  if (!info.title) problems.push(`[seo] ${route} has no title`);
  if (info.h1.length !== 1) problems.push(`[a11y] ${route} has ${info.h1.length} h1`);
  if (!info.canonical && route !== '/404.html') problems.push(`[seo] ${route} has no canonical`);
  if (!info.desc) problems.push(`[seo] ${route} has no description`);
  if (info.fonts < 2) problems.push(`[font] ${route} loaded ${info.fonts} fonts`);

  console.log(
    `${route.padEnd(32)} h1=${info.h1.length} ld=${info.jsonld} theme=${info.theme} ` +
      `fonts=${info.fonts} bg=${info.bg}`
  );

  await page.screenshot({ path: `${outDir}/${name}.png`, fullPage: true });
}

// ── Client-side navigation ───────────────────────────────────────────────────
await page.goto(base + '/', { waitUntil: 'networkidle' });
await waitHydrated(page);
await page.click('a[href="/projects/"]');
await page.waitForTimeout(600);
const afterNav = await page.evaluate(() => ({
  url: location.pathname,
  title: document.title,
  canonical: document.querySelector('link[rel="canonical"]')?.href,
  h1: document.querySelector('h1')?.textContent.trim(),
  focus: document.activeElement?.id,
}));
console.log('client nav →', JSON.stringify(afterNav));
if (afterNav.url !== '/projects/') problems.push('[router] click did not change the URL');
if (!afterNav.canonical?.endsWith('/projects/')) problems.push('[router] canonical not updated');
if (!afterNav.title.toLowerCase().includes('project')) problems.push('[router] title not updated');

await page.click('a[href="/projects/dtact/"]');
await page.waitForTimeout(600);
const deep = await page.evaluate(() => ({
  url: location.pathname,
  title: document.title,
  h1: document.querySelector('h1')?.textContent.trim(),
}));
console.log('client nav →', JSON.stringify(deep));

// Back button
await page.goBack();
await page.waitForTimeout(500);
console.log('after back →', await page.evaluate(() => location.pathname));

// ── Theme toggle ────────────────────────────────────────────────────────────
await page.goto(base + '/', { waitUntil: 'networkidle' });
await waitHydrated(page);
const before = await page.evaluate(() => document.documentElement.getAttribute('data-theme'));
await page.click('.theme-toggle');
await page.waitForTimeout(400);
const after = await page.evaluate(() => ({
  theme: document.documentElement.getAttribute('data-theme'),
  stored: localStorage.getItem('apich-theme'),
  bg: getComputedStyle(document.body).backgroundColor,
}));
console.log(`theme ${before} → ${JSON.stringify(after)}`);
if (before === after.theme) problems.push('[theme] toggle did not change the theme');
await page.screenshot({ path: `${outDir}/home-dark.png`, fullPage: true });
await page.goto(base + '/projects/dtact/', { waitUntil: 'networkidle' });
await page.screenshot({ path: `${outDir}/dtact-dark.png`, fullPage: true });

// ── Copy button ─────────────────────────────────────────────────────────────
await page.goto(base + '/projects/dtact/', { waitUntil: 'networkidle' });
await waitHydrated(page);
const copyBtn = await page.$('[data-copy]');
if (copyBtn) {
  await copyBtn.click();
  await page.waitForTimeout(300);
  const label = await page.evaluate(() => document.querySelector('[data-copy] span')?.textContent);
  console.log('copy button →', label);
} else {
  problems.push('[copy] no copy button on the dtact page');
}

// ── Mobile ──────────────────────────────────────────────────────────────────
const mobile = await newPage(400, 860);
await mobile.goto(base + '/', { waitUntil: 'networkidle' });
await waitHydrated(mobile);
await mobile.screenshot({ path: `${outDir}/home-mobile.png`, fullPage: true });
await mobile.click('.nav-burger');
await mobile.waitForTimeout(500);
const drawerOpen = await mobile.evaluate(() => {
  const d = document.getElementById('nav-drawer');
  const r = d.getBoundingClientRect();
  return { open: d.classList.contains('open'), height: Math.round(r.height), top: Math.round(r.top) };
});
console.log('drawer →', JSON.stringify(drawerOpen));
if (!drawerOpen.open || drawerOpen.height < 100) problems.push('[mobile] drawer did not open');
await mobile.screenshot({ path: `${outDir}/home-mobile-menu.png` });
await mobile.click('#nav-drawer a[href="/projects/"]');
await mobile.waitForTimeout(600);
const mobileNav = await mobile.evaluate(() => ({
  url: location.pathname,
  open: document.getElementById('nav-drawer').classList.contains('open'),
}));
console.log('mobile nav →', JSON.stringify(mobileNav));
if (mobileNav.open) problems.push('[mobile] drawer stayed open after navigation');

await browser.close();

console.log('\n── problems ──');
if (problems.length === 0) console.log('none');
else problems.forEach((p) => console.log('• ' + p));
console.log(`\n${problems.length} problem(s)`);
process.exit(problems.length ? 1 : 0);
