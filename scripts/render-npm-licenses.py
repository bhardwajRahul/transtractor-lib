#!/usr/bin/env python3
"""Render license-checker JSON output as GitHub-flavoured Markdown."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from typing import Any

ALLOWED_LICENSES = {
    "0BSD",
    "Apache-2.0",
    "BSD",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "MIT",
    "Python-2.0",
}


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    text = path.read_text(encoding="utf-8").strip()
    if not text:
        return None
    data = json.loads(text)
    return data if isinstance(data, dict) else None


def _normalize(license_name: str) -> list[str]:
    parts = license_name.replace("(", "").replace(")", "")
    for token in [" OR ", " AND ", "/", ","]:
        parts = parts.replace(token, "|")
    return [chunk.strip() for chunk in parts.split("|") if chunk.strip()]


def _is_allowed(license_name: str) -> bool:
    if not license_name:
        return False
    tokens = _normalize(license_name)
    return all(token in ALLOWED_LICENSES for token in tokens)


def render(data: dict[str, Any] | None) -> tuple[str, int]:
    lines = ["## JavaScript License Audit (license-checker)", ""]

    if data is None:
        lines.append("_License check produced no output._")
        return "\n".join(lines), 0

    findings: list[dict[str, str]] = []
    for package_id, metadata in data.items():
        if not isinstance(metadata, dict):
            continue
        license_name = str(metadata.get("licenses") or "unknown")
        if _is_allowed(license_name):
            continue
        findings.append(
            {
                "package": package_id,
                "license": license_name,
                "repository": str(metadata.get("repository") or ""),
            }
        )

    if not findings:
        lines.append("✅ No disallowed licenses found.")
        return "\n".join(lines), 0

    total = len(findings)
    plural = "s" if total != 1 else ""
    lines.append(f"**{total} disallowed license{plural}** found.")
    lines.append("")
    lines.append("| Package | License | Repository |")
    lines.append("| --- | --- | --- |")

    for finding in findings:
        repo = finding["repository"] or "N/A"
        lines.append(f"| `{finding['package']}` | `{finding['license']}` | {repo} |")

    return "\n".join(lines), len(findings)


def write_github_output(name: str, value: str) -> None:
    output_path = os.environ.get("GITHUB_OUTPUT")
    if output_path:
        with open(output_path, "a", encoding="utf-8") as fh:
            fh.write(f"{name}={value}\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="Path to license-checker JSON file")
    parser.add_argument("output", type=Path, help="Path to output Markdown file")
    args = parser.parse_args()

    body, finding_count = render(load_json(args.input))
    args.output.write_text(body + "\n", encoding="utf-8")

    has_findings = "true" if finding_count > 0 else "false"
    write_github_output("has_findings", has_findings)


if __name__ == "__main__":
    main()
