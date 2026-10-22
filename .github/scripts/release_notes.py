"""Extract one version's GitHub Release body from the cumulative changelog."""

import re
import sys
from pathlib import Path


VERSION_HEADING = re.compile(r"^## ([^\s(]+)(?:\s|$)")
SUBHEADING = re.compile(r"^(#{3,6})(?= )", re.MULTILINE)
VERSION_TAG = re.compile(r"v?[0-9]+\.[0-9]+\.[0-9]+")


def release_notes(changelog: str, tag: str) -> str:
    if not VERSION_TAG.fullmatch(tag):
        raise ValueError(f"expected a version tag in vX.Y.Z or X.Y.Z form, got {tag}")
    version = tag.removeprefix("v")
    lines = changelog.splitlines(keepends=True)
    headings = [
        (index, match.group(1).strip("[]"))
        for index, line in enumerate(lines)
        if (match := VERSION_HEADING.match(line))
    ]
    matching = [index for index, heading_version in headings if heading_version == version]
    if len(matching) != 1:
        raise ValueError(f"expected exactly one CHANGELOG.md section for {tag}, found {len(matching)}")

    start = matching[0]
    end = next((index for index, _ in headings if index > start), len(lines))
    body = "".join(lines[start + 1 : end]).strip()
    if not body:
        raise ValueError(f"CHANGELOG.md section for {tag} is empty")

    # The version heading becomes the GitHub Release title; promote its child headings.
    return SUBHEADING.sub(lambda match: "#" * (len(match.group(1)) - 1), body) + "\n"


if __name__ == "__main__":
    if len(sys.argv) != 4:
        raise SystemExit("usage: release_notes.py TAG CHANGELOG.md OUTPUT.md")
    try:
        Path(sys.argv[3]).write_text(release_notes(Path(sys.argv[2]).read_text(), sys.argv[1]))
    except ValueError as error:
        raise SystemExit(str(error)) from error
