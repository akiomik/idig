"""Checks for the release body passed to actions-rust-release/publish."""

import unittest
from pathlib import Path

from release_notes import release_notes


class ReleaseNotesTest(unittest.TestCase):
    def test_extracts_only_the_tagged_version_and_promotes_subheadings(self):
        changelog = "# Changelog\n\n## 0.3.0 (2026-01-01)\n\n### Features\n\n* New\n\n## 0.2.0\n\n### Fixes\n\n* Old\n"
        self.assertEqual("## Features\n\n* New\n", release_notes(changelog, "v0.3.0"))

    def test_current_release_has_only_its_own_changes(self):
        changelog = (Path(__file__).resolve().parents[2] / "CHANGELOG.md").read_text()
        body = release_notes(changelog, "v0.2.0")
        self.assertTrue(body.startswith("## ⚠️ BREAKING CHANGES\n"))
        self.assertIn("## Other Changes\n", body)
        self.assertNotIn("Initial release!", body)
        self.assertNotIn("## 0.2.0", body)

    def test_fails_for_a_missing_version(self):
        with self.assertRaisesRegex(ValueError, "found 0"):
            release_notes("# Changelog\n\n## 0.2.0\n\n* Existing\n", "v0.3.0")

    def test_extracts_prerelease_notes_when_the_heading_matches(self):
        changelog = "# Changelog\n\n## 1.0.0-alpha.beta\n\n* Preview\n"
        self.assertEqual("* Preview\n", release_notes(changelog, "v1.0.0-alpha.beta"))

    def test_accepts_versions_with_and_without_v_prefix(self):
        changelog = "# Changelog\n\n## 0.3.0\n\n* New\n"
        self.assertEqual("* New\n", release_notes(changelog, "v0.3.0"))
        self.assertEqual("* New\n", release_notes(changelog, "0.3.0"))


if __name__ == "__main__":
    unittest.main()
