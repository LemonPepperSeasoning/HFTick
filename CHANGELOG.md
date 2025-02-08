# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.2.1] - 2025-02-08

Allow multiple subroutine to be scheduled & managed.

### Added

- `Scheduler` class that can manage & run multiple subroutines.
- can terminate 1 task via terminate() function
- can terminate all task via shutdown() function

### Changed

- Removed `run_scheduler` function in favour of `Scheduler` class

### Fixed

## [0.1.0] - 2025-02-07

Bare minimum scheduler.

### Added

- `run_scheduler` function that runs a provided function every N seconds.

### Changed

### Fixed

## [Unreleased] - 2024-03-16

Init. Basic repo setup

### Added

### Changed

### Fixed
