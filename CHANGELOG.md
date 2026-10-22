# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-10-22

### Fixed

- Restore macOS x86_64 release archives alongside arm64 archives, and publish
  the GitHub Release only after both have been packaged
  ([#78](https://github.com/akiomik/idig/pull/78),
  [#79](https://github.com/akiomik/idig/pull/79)).
- Publish release notes for the tagged version only, instead of the entire
  changelog ([#78](https://github.com/akiomik/idig/pull/78)).

### Security

- Update TLS certificate trust dependencies following the eMudhra CA compromise
  ([#73](https://github.com/akiomik/idig/pull/73)).
- Address RustSec advisories by updating `plist` and removing unused SeaORM
  default features and their MySQL, RSA, and rkyv dependency paths
  ([#77](https://github.com/akiomik/idig/pull/77)).

## [0.2.0] - 2025-11-28

### Added

- Add the `list` (`ls`) command to enumerate local iPhone backups
  ([#35](https://github.com/akiomik/idig/pull/35)).

### Changed

- Move `--backup-dir` from a global option to the `search` and `extract`
  subcommands. Existing commands must place it after the subcommand
  ([#24](https://github.com/akiomik/idig/pull/24)).
- Improve command output for search and extraction
  ([#4](https://github.com/akiomik/idig/pull/4)).

## [0.1.0] - 2025-07-18

### Added

- Initial release with `search` and `extract` commands for unencrypted iPhone
  backups.

[Unreleased]: https://github.com/akiomik/idig/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/akiomik/idig/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/akiomik/idig/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/akiomik/idig/releases/tag/v0.1.0
