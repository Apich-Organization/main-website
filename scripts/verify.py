#!/usr/bin/env python3
"""Structural checks for the generated site.

Run after `scripts/build.sh`:

    python3 scripts/verify.py dist

Fails with a non-zero exit code when a page is missing required metadata, when
an internal link points at nothing, or when the machine-readable files are
absent. It is intentionally dependency-free so CI can run it with a stock
Python 3.
"""

from __future__ import annotations

import json
import os
import re
import sys
from html.parser import HTMLParser
from urllib.parse import urlparse

REQUIRED_ROOT = [
    "robots.txt",
    "sitemap.xml",
    "llms.txt",
    "llms-full.txt",
    "CNAME",
    "404.html",
    "site.webmanifest",
    "data/projects.json",
    "data/site.json",
    "blog/feed.xml",
    "favicon.svg",
    "favicon.ico",
    "assets/img/icon-192.png",
    "assets/img/icon-512.png",
    "assets/img/apple-touch-icon.png",
    "assets/security-pgp.asc",
    ".well-known/security.txt",
]

META_REQUIRED = [
    'name="description"',
    'rel="canonical"',
    'property="og:title"',
    'property="og:description"',
    'property="og:image"',
    'property="og:url"',
    'name="twitter:card"',
    'name="viewport"',
]


class Page(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.title = ""
        self.in_title = False
        self.h1 = 0
        self.html_lang = None
        self.metas: list[str] = []
        self.links: list[str] = []
        self.imgs: list[tuple[str, str | None]] = []
        self.jsonld: list[str] = []
        self.in_jsonld = False
        self.jsonld_buf: list[str] = []

    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if tag == "html":
            self.html_lang = a.get("lang")
        elif tag == "title":
            self.in_title = True
        elif tag == "h1":
            self.h1 += 1
        elif tag == "meta":
            self.metas.append(
                " ".join(f'{k}="{v}"' for k, v in attrs if v is not None)
            )
        elif tag == "link":
            if a.get("href"):
                self.links.append(a["href"])
            self.metas.append(
                " ".join(f'{k}="{v}"' for k, v in attrs if v is not None)
            )
        elif tag == "a":
            if a.get("href"):
                self.links.append(a["href"])
        elif tag == "img":
            self.imgs.append((a.get("src", ""), a.get("alt")))
        elif tag == "script" and a.get("type") == "application/ld+json":
            self.in_jsonld = True
            self.jsonld_buf = []

    def handle_endtag(self, tag):
        if tag == "title":
            self.in_title = False
        elif tag == "script" and self.in_jsonld:
            self.in_jsonld = False
            self.jsonld.append("".join(self.jsonld_buf))

    def handle_data(self, data):
        if self.in_title:
            self.title += data
        if self.in_jsonld:
            self.jsonld_buf.append(data)


def internal_target(href: str) -> str | None:
    """Maps an internal href to a path inside dist, or None if external."""
    if href.startswith(("http://", "https://", "mailto:", "tel:", "#", "data:")):
        return None
    if href.startswith("//"):
        return None
    path = urlparse(href).path
    if not path.startswith("/"):
        return None
    return path


def main() -> int:
    root = sys.argv[1] if len(sys.argv) > 1 else "dist"
    errors: list[str] = []
    warnings: list[str] = []

    for rel in REQUIRED_ROOT:
        if not os.path.isfile(os.path.join(root, rel)):
            errors.append(f"missing required file: {rel}")

    # The published key has to be a key, not a placeholder or an empty file.
    key_path = os.path.join(root, "assets/security-pgp.asc")
    if os.path.isfile(key_path):
        with open(key_path, encoding="utf-8") as fh:
            key = fh.read()
        if not key.startswith("-----BEGIN PGP PUBLIC KEY BLOCK-----"):
            errors.append("security-pgp.asc does not start with a PGP public key header")
        if "-----END PGP PUBLIC KEY BLOCK-----" not in key:
            errors.append("security-pgp.asc is truncated (no end marker)")
        if len(key) < 500:
            errors.append("security-pgp.asc is suspiciously small")

    # RFC 9116 requires Contact and Expires, and recommends Canonical.
    sec_path = os.path.join(root, ".well-known/security.txt")
    if os.path.isfile(sec_path):
        with open(sec_path, encoding="utf-8") as fh:
            sec = fh.read()
        for field in ("Contact: mailto:", "Expires:", "Canonical:"):
            if field not in sec:
                errors.append(f"security.txt is missing {field!r}")

    for rel in ("data/projects.json", "data/site.json", "site.webmanifest"):
        path = os.path.join(root, rel)
        if os.path.isfile(path):
            try:
                with open(path, encoding="utf-8") as fh:
                    json.load(fh)
            except json.JSONDecodeError as exc:
                errors.append(f"{rel}: invalid JSON ({exc})")

    # llms.txt is a link list for agents; a dead entry there is worse than none.
    llms_path = os.path.join(root, "llms.txt")
    if os.path.isfile(llms_path):
        with open(llms_path, encoding="utf-8") as fh:
            llms = fh.read()
        for href in re.findall(r"\]\((https://apich\.org[^)]*)\)", llms):
            target = urlparse(href).path or "/"
            local = os.path.join(root, target.lstrip("/"))
            if target.endswith("/"):
                local = os.path.join(local, "index.html")
            if not os.path.exists(local):
                errors.append(f"llms.txt: dead link {href}")

    pages: list[str] = []
    for dirpath, _dirnames, filenames in os.walk(root):
        if "index.html" in filenames:
            pages.append(os.path.join(dirpath, "index.html"))
    pages.sort()

    if len(pages) < 10:
        errors.append(f"only {len(pages)} pages generated — expected at least 10")

    titles: dict[str, str] = {}
    descriptions: dict[str, str] = {}
    checked_links: set[str] = set()

    for page_path in pages:
        rel = os.path.relpath(page_path, root).replace("\\", "/")
        with open(page_path, encoding="utf-8") as fh:
            html = fh.read()

        parser = Page()
        parser.feed(html)

        if parser.html_lang != "en":
            errors.append(f"{rel}: <html lang> is {parser.html_lang!r}, expected 'en'")
        if not parser.title:
            errors.append(f"{rel}: missing <title>")
        elif not 15 <= len(parser.title) <= 75:
            warnings.append(f"{rel}: title length {len(parser.title)}: {parser.title!r}")
        if parser.title in titles:
            errors.append(f"{rel}: duplicate title with {titles[parser.title]}")
        titles[parser.title] = rel

        if parser.h1 != 1:
            errors.append(f"{rel}: {parser.h1} <h1> elements, expected exactly 1")

        joined = "\n".join(parser.metas)
        for needle in META_REQUIRED:
            if needle not in joined:
                errors.append(f"{rel}: missing {needle}")

        desc = re.search(r'name="description" content="([^"]*)"', html)
        if desc:
            d = desc.group(1)
            if not 60 <= len(d) <= 180:
                warnings.append(f"{rel}: description length {len(d)}")
            if d in descriptions:
                errors.append(f"{rel}: duplicate description with {descriptions[d]}")
            descriptions[d] = rel

        for block in parser.jsonld:
            try:
                json.loads(block)
            except json.JSONDecodeError as exc:
                errors.append(f"{rel}: invalid JSON-LD ({exc})")

        for src, alt in parser.imgs:
            if alt is None:
                errors.append(f"{rel}: <img src={src!r}> has no alt attribute")

        for href in parser.links:
            target = internal_target(href)
            if target is None or target in checked_links:
                continue
            checked_links.add(target)
            local = os.path.join(root, target.lstrip("/"))
            if target.endswith("/"):
                local = os.path.join(local, "index.html")
            if not os.path.exists(local):
                errors.append(f"{rel}: broken internal link {href}")

    # Sitemap must list every generated page.
    sitemap_path = os.path.join(root, "sitemap.xml")
    if os.path.isfile(sitemap_path):
        with open(sitemap_path, encoding="utf-8") as fh:
            sitemap = fh.read()
        for page_path in pages:
            rel = os.path.relpath(os.path.dirname(page_path), root).replace("\\", "/")
            if rel == ".":
                continue
            if f"/{rel}/" not in sitemap:
                errors.append(f"sitemap.xml is missing /{rel}/")

    for w in warnings:
        print(f"warn  {w}")
    for e in errors:
        print(f"error {e}")

    print(f"\nchecked {len(pages)} pages, {len(checked_links)} internal links")
    if errors:
        print(f"FAILED with {len(errors)} error(s)")
        return 1
    print("OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
