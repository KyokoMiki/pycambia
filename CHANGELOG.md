# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/KyokoMiki/pycambia/compare/v0.2.0...HEAD)

## [v0.2.0](https://github.com/KyokoMiki/pycambia/compare/v0.1.0...v0.2.0) - 2026-02-13

### Added

- Strongly typed API with comprehensive type hints and type stubs (`_cambia.pyi`).
- Enum-based return types for ripper types, media types, read modes, and other categorical data.
- Python 3.13t and 3.14t (free-threaded) support with GIL-free mode enabled.
- PathLike support for `parse_log_file()`, now accepts `pathlib.Path` objects in addition to strings.

### Changed

- **BREAKING**: Complete API redesign with typed objects instead of dictionaries.
  - `parse_file()` renamed to `parse_log_file()` and returns `CambiaResponse` object.
  - `parse_content()` renamed to `parse_log_content()` and returns `CambiaResponse` object.
  - `supported_rippers()` renamed to `get_supported_rippers()` and returns `list[Ripper]` enum members.
  - Removed `LogParser` class - use module-level functions directly.
  - All parsed data now uses typed objects with attributes instead of dictionary keys.
  
- **BREAKING**: Error handling changed from `{"success": bool, "error": str}` to raising Python exceptions (`OSError`, `ValueError`, `TypeError`).
- **BREAKING**: Minimum Python version raised from 3.8 to 3.10.
- CI/CD migrated from `python-package.yml` to `build-and-publish.yml` with maturin-based builds.

### Removed

- **BREAKING**: Dictionary-based return format - all data now returned as typed objects.
- **BREAKING**: `LogParser` class interface.
- **BREAKING**: Success/error wrapper dictionary - functions now raise exceptions on error.

### Fixed

- CI pipeline now uses `--no-index` flag for pip install to prevent fallback to PyPI.
- Proper binary log file handling with `.gitattributes`.

### Performance

- Improved serialization performance by switching from `serde_json` to `pythonize`, eliminating intermediate JSON serialization step and directly converting Rust types to Python objects.

**Full Changelog**: https://github.com/KyokoMiki/pycambia/commits/v0.2.0

## [v0.1.0](https://github.com/KyokoMiki/pycambia/commits/v0.1.0) - 2025-08-16

### Added

- Python bindings for cambia via PyO3 with `pythonize` serialization.
- Type stubs (`_cambia.pyi`) and PEP 561 `py.typed` marker for static type checking support.
- pytest test suite with EAC and XLD log fixtures.
- CI workflow for Python package building and publishing.

**Full Changelog**: https://github.com/KyokoMiki/pycambia/commits/v0.1.0
