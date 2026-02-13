# pycambia

[![PyPI version](https://badgen.net/pypi/v/pycambia)](https://pypi.org/project/pycambia/)
[![PyPI - Python Version](https://badgen.net/pypi/python/pycambia)](https://pypi.org/project/pycambia/)
[![CI](https://github.com/KyokoMiki/pycambia/actions/workflows/build-and-publish.yml/badge.svg)](https://github.com/KyokoMiki/pycambia/actions/workflows/build-and-publish.yml)
[![License: MIT](https://badgen.net/github/license/KyokoMiki/pycambia)](https://github.com/KyokoMiki/pycambia/blob/master/LICENSE)

Python bindings for [cambia](https://github.com/arg274/cambia), a CD rip log parser and evaluator written in Rust. Parse and score rip logs from EAC, XLD, whipper, and more.

## Installation

```sh
pip install pycambia
```

Requires Python 3.10+. Pre-built wheels are available for Linux (x86_64, aarch64, armv7, s390x, ppc64le), Windows (x64, ARM64), and macOS (x86_64, aarch64).

## Quick Start

```python
import cambia

# Parse a log file
result = cambia.parse_log_file("/path/to/eac.log")

# Access parsed data
log = result.parsed.parsed_logs[0]
print(f"Ripper: {log.ripper.name} v{log.ripper_version}")
print(f"Artist: {log.release_info.artist}")
print(f"Album: {log.release_info.title}")
print(f"Tracks: {len(log.tracks)}")

# Access evaluation score
score = result.evaluation_combined[0].combined_score
print(f"Score: {score}")
```

## API Reference

### `cambia.parse_log_file(path)`

Parse a CD rip log file from disk.

- **path** (`str | PathLike`) – Path to the log file. Accepts string paths or PathLike objects (e.g., `pathlib.Path`)
- **Returns**: `CambiaResponse`
- **Raises**: `OSError` if the file cannot be read, `ValueError` if parsing fails, `TypeError` if path is not `str` or PathLike

```python
# Using string path
result = cambia.parse_log_file("/path/to/eac.log")

# Using pathlib.Path
from pathlib import Path
result = cambia.parse_log_file(Path("/path/to/eac.log"))
```

### `cambia.parse_log_content(content)`

Parse log content from a string or bytes.

- **content** (`str | bytes`) – Log file content. When `bytes`, the encoding is auto-detected by cambia-core.
- **Returns**: `CambiaResponse`
- **Raises**: `ValueError` if parsing fails, `TypeError` if content is not `str` or `bytes`

```python
# From string (converted to UTF-8 bytes)
with open("/path/to/xld.log", "r", encoding="utf-8") as f:
    result = cambia.parse_log_content(f.read())

# From bytes (auto-detects encoding)
with open("/path/to/eac.log", "rb") as f:
    result = cambia.parse_log_content(f.read())
```

### `cambia.get_supported_rippers()`

Get list of supported CD ripper types.

- **Returns**: `list[Ripper]` – List of `Ripper` enum values

```python
>>> rippers = cambia.get_supported_rippers()
>>> [r.name for r in rippers]
['EAC', 'XLD', 'Whipper', 'CueRipper']
```

## Return Types

All parsing functions return a `CambiaResponse` object with fully typed attributes.

### `CambiaResponse`

| Attribute              | Type                       | Description                  |
| ---------------------- | -------------------------- | ---------------------------- |
| `id`                   | `bytes`                    | Unique identifier            |
| `parsed`               | `ParsedLogCombined`        | Parsed log data              |
| `evaluation_combined`  | `list[EvaluationCombined]` | Evaluation / scoring results |

### `ParsedLogCombined`

| Attribute     | Type             | Description              |
| ------------- | ---------------- | ------------------------ |
| `encoding`    | `str`            | Detected file encoding   |
| `parsed_logs` | `list[ParsedLog]`| Parsed log entries       |

### `ParsedLog`

| Attribute            | Type               | Description                           |
| -------------------- | ------------------ | ------------------------------------- |
| `ripper`             | `Ripper`           | Ripper software                       |
| `ripper_version`     | `str`              | Ripper version string                 |
| `release_info`       | `ReleaseInfo`      | Album artist and title                |
| `language`           | `str`              | Log language                          |
| `read_offset`        | `int \| None`      | Read offset value                     |
| `combined_rw_offset` | `int \| None`      | Combined read/write offset            |
| `drive`              | `str`              | CD drive model                        |
| `media_type`         | `MediaType`        | Media type (Pressed, CD-R, etc.)      |
| `accurate_stream`    | `Quartet`          | Accurate stream setting               |
| `defeat_audio_cache` | `Quartet`          | Audio cache defeat setting            |
| `use_c2`             | `Quartet`          | C2 error correction setting           |
| `overread`           | `Quartet`          | Overread setting                      |
| `fill_silence`       | `Quartet`          | Fill silence setting                  |
| `delete_silence`     | `Quartet`          | Delete silence setting                |
| `use_null_samples`   | `Quartet`          | Null samples setting                  |
| `test_and_copy`      | `Quartet`          | Test & copy mode                      |
| `normalize`          | `Quartet`          | Normalize setting                     |
| `read_mode`          | `ReadMode`         | Read mode (Secure, Paranoid, etc.)    |
| `gap_handling`       | `Gap`              | Gap handling method                   |
| `checksum`           | `Checksum`         | Overall log checksum                  |
| `toc`                | `Toc`              | Table of contents                     |
| `tracks`             | `list[TrackEntry]` | Individual track results              |
| `id3_enabled`        | `Quartet`          | ID3 tagging setting                   |
| `audio_encoder`      | `list[str]`        | Audio encoder information             |

### Enums

All enums have `name` (variant name) and `value` (human-readable label) attributes:

```python
>>> cambia.Ripper.EAC.name
'EAC'
>>> cambia.Ripper.EAC.value
'Exact Audio Copy'
```

| Enum               | Variants                                                                         |
| ------------------ | -------------------------------------------------------------------------------- |
| `Ripper`           | EAC, XLD, Whipper, CueRipper, DBPA, CyanRip, EZCD, Morituri, Rip, FreAc, Other |
| `MediaType`        | Pressed, CDR, Other, Unknown                                                     |
| `Quartet`          | True, False, Unknown, Unsupported                                                |
| `ReadMode`         | Secure, Paranoid, Fast, Burst, Unknown                                           |
| `Gap`              | Append, AppendNoHtoa, AppendUndetected, Prepend, Discard, Unknown, Inapplicable  |
| `Integrity`        | Match, Mismatch, Unknown                                                         |
| `AccurateRipStatus`| Match, Mismatch, Offsetted, NotFound, Disabled                                   |
| `EvaluatorType`    | Cambia, RED, OPS                                                                 |

## Examples

### Basic Log Parsing

```python
import cambia

result = cambia.parse_log_file("/path/to/eac.log")
log = result.parsed.parsed_logs[0]

# Ripper info
print(f"Ripper: {log.ripper.name} v{log.ripper_version}")
print(f"Drive: {log.drive}")
print(f"Read mode: {log.read_mode.name}")
print(f"Read offset: {log.read_offset}")

# Album info
print(f"Artist: {log.release_info.artist}")
print(f"Album: {log.release_info.title}")
print(f"Encoding: {result.parsed.encoding}")

# Checksum
print(f"Checksum integrity: {log.checksum.integrity.name}")
```

### Track Details

```python
result = cambia.parse_log_file("/path/to/eac.log")
log = result.parsed.parsed_logs[0]

for track in log.tracks:
    print(f"Track {track.num}:")
    print(f"  Files: {track.filenames}")
    print(f"  Peak level: {track.peak_level}")
    print(f"  Speed: {track.extraction_speed}x")

    # Test & copy verification
    tc = track.test_and_copy
    print(f"  T&C integrity: {tc.integrity.name}")

    # AccurateRip
    for ar in track.ar_info:
        print(f"  AR: {ar.status.name} (v{ar.version})")
        if ar.confidence:
            print(f"    Confidence: {ar.confidence.matching}/{ar.confidence.total}")
```

### Evaluation Score

```python
result = cambia.parse_log_file("/path/to/eac.log")

for eval_combined in result.evaluation_combined:
    print(f"Evaluator: {eval_combined.evaluator.name}")
    print(f"Score: {eval_combined.combined_score}")

    for evaluation in eval_combined.evaluations:
        print(f"  Log score: {evaluation.score}")
        for unit in evaluation.evaluation_units:
            data = unit.data
            print(f"    [{data.field.name}] {data.message}")
```

### Error Handling

```python
import cambia

# File not found
try:
    result = cambia.parse_log_file("nonexistent.log")
except OSError as e:
    print(f"File error: {e}")

# Invalid or unsupported content
try:
    result = cambia.parse_log_content("not a valid log")
except ValueError as e:
    print(f"Parse error: {e}")

# Wrong argument type
try:
    result = cambia.parse_log_content(12345)
except TypeError as e:
    print(f"Type error: {e}")
```

### Parsing Bytes

Reading as bytes lets cambia-core handle encoding detection automatically:

```python
# Read as bytes for automatic encoding detection
with open("/path/to/eac.log", "rb") as f:
    raw = f.read()
result = cambia.parse_log_content(raw)

print(f"Detected encoding: {result.parsed.encoding}")
```

## Supported Rippers

| Ripper    | Status            | Description                                     |
| --------- | ----------------- | ----------------------------------------------- |
| EAC       | ✅ Stable          | Exact Audio Copy – Windows CD ripper            |
| XLD       | ✅ Stable          | X Lossless Decoder – macOS CD ripper            |
| Whipper   | ✅ Stable          | Command-line CD ripper (successor to morituri)  |
| CueRipper | ⚠️ Experimental   | Windows CD ripper                               |

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Python 3.10+](https://www.python.org/downloads/)
- [uv](https://docs.astral.sh/uv/) (Python package manager)
- [maturin](https://github.com/PyO3/maturin) (build tool for PyO3, installed via uv)

### Setup

```sh
git clone https://github.com/KyokoMiki/pycambia.git
cd pycambia

# Install development dependencies
uv sync --group dev

# Build and install the extension in development mode
uv run maturin develop

# Run tests
uv run pytest -v
```

### Building for Distribution

```sh
# Build wheel for current platform
uv run maturin build --release

# Build and publish to PyPI
uv run maturin publish
```

### Features

The project uses Cargo features to control functionality:

- `experimental_rippers` (default): Enables support for experimental rippers like CueRipper

To build without experimental rippers:

```sh
uv run maturin develop --no-default-features
```

## License

[MIT](https://github.com/KyokoMiki/pycambia/blob/master/LICENSE)