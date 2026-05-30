# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.1] - 2026-05-30

### Changed

- Grant pull-requests write permission in binary-check workflow

- **cliff:** Remove blank line between version heading and group list

- Remove explicit version fields from workspace dependencies

- Add workflow to tag releases and fix condition syntax

- Replace release-plz with git-cliff for release automation

- Disable publishing and git releases in release-plz config

- Add git_release flag to release-plz config

- Refactor workspace dependencies to use centralized version

- Add explicit version field to all workspace dependency references

- Prevent accidental publication of all crates

- Fix changelog paths in release-plz config

- Add format flag to `/usr/bin/time` in compile time jobs

- Revamp workflows, add binary-check and AUTOMATIONS.md


### Fixed

- **ci:** Correct action input names and remove stale comments

- Use version from Cargo.toml when generating changelog

- **cliff:** Use multiline strings and optional timestamp in template

- **release:** Determine version bump from commit history

- Add Hippocratic-3.0 license allowance and clarifications

- Remove release_always config from release-plz.toml

- Downgrade actions/cache from v6 to v5

- Give release.yml write permissions

## [v0.1.0] - 2026-05-29

### Added

- Add release pipeline and rename binary to opc_ua_for_plc

- **server:** Inline tests into native.rs and document test location

- **server:** Enable OPC UA secure channel and security policies

- **core-model:** Make plc_name required on TagDefinition

- Docker-based end-to-end tests

- Initial release


### Changed

- **release:** Update release-plz config

- Remove deprecated shutdown/event bridge and health map

- Update actions/checkout to v5 and improve binary baseline cache keys

- Simplify match guards in data type validation

- Update GitHub Actions to latest versions


### Documentation

- **readme:** Remove duplicated UA-METHOD feature

- **adr:** Add write confirmation and sync-to-async bridge decisions


### Fixed

- **config:** Make unit_id optional with default fallback

- **driver:** Move context after write handling in read cycle

- Fix(driver/fins): adjusting byte offsets
to match the FINS protocol layout

- Enable release-plz by removing `publish = false` from crates


### Other

- Initial commit

