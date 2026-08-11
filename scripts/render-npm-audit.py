#!/usr/bin/env python3
"""Render npm audit --json output as GitHub-flavoured Markdown."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from typing import Any


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    text = path.read_text(encoding="utf-8").strip()
    if not text:
        return None
    return json.loads(text)


def _extract_vulns(data: dict[str, Any]) -> list[dict[str, Any]]:
    vulns = data.get("vulnerabilities", {})
    if not isinstance(vulns, dict):
        return []

    findings: list[dict[str, Any]] = []
    for package, info in vulns.items():
        if not isinstance(info, dict):
            continue

        via = info.get("via", [])
        if not isinstance(via, list):
            via = [via]

        for entry in via:
            if isinstance(entry, str):
                findings.append(
                    {
                        "package": package,
                        "severity": info.get("severity", "unknown"),
                        "id": entry,
                        "title": entry,
                        "range": info.get("range", ""),
                        "fix": info.get("fixAvailable", False),
                    }
                )
                continue

            if not isinstance(entry, dict):
                continue

            findings.append(
                {
                    "package": package,
                    "severity": entry.get("severity")
                    or info.get("severity", "unknown"),
                    "id": entry.get("source") or entry.get("url") or "N/A",
                    "title": entry.get("title") or "N/A",
                    "range": entry.get("range") or info.get("range", ""),
                    "fix": info.get("fixAvailable", False),
                }
            )

    return findings


def render(data: dict[str, Any] | None) -> tuple[str, int]:
    lines = ["## JavaScript Dependency Audit (npm audit)", ""]

    if data is None:
        lines.append("_Audit produced no output._")
        return "\n".join(lines), 0

    findings = _extract_vulns(data)

    if not findings:
        lines.append("✅ No known vulnerabilities found.")
        return "\n".join(lines), 0

    total = len(findings)
    plural = "s" if total != 1 else ""
    lines.append(f"**{total} known vulnerability{plural}** found.")
    lines.append("")
    lines.append("| Package | Severity | Advisory | Affected Range | Fix Available |")
    lines.append("| --- | --- | --- | --- | --- |")

    for finding in findings:
        severity = str(finding["severity"]).upper()
        advisory = str(finding["id"])
        title = str(finding["title"])
        range_ = str(finding["range"])
        fix = finding["fix"]
        if isinstance(fix, dict):
            fix_state = fix.get("name") or "yes"
        elif fix is True:
            fix_state = "yes"
        elif fix:
            fix_state = str(fix)
        else:
            fix_state = "no"
        lines.append(
            f"| `{finding['package']}` | `{severity}` | {advisory} ({title}) | "
            f"`{range_}` | `{fix_state}` |"
        )

    return "\n".join(lines), len(findings)


def write_github_output(name: str, value: str) -> None:
    output_path = os.environ.get("GITHUB_OUTPUT")
    if output_path:
        with open(output_path, "a", encoding="utf-8") as fh:
            fh.write(f"{name}={value}\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="Path to npm audit JSON file")
    parser.add_argument("output", type=Path, help="Path to output Markdown file")
    args = parser.parse_args()

    body, finding_count = render(load_json(args.input))
    args.output.write_text(body + "\n", encoding="utf-8")

    has_findings = "true" if finding_count > 0 else "false"
    write_github_output("has_findings", has_findings)


if __name__ == "__main__":
    main()
