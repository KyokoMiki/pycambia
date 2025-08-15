# pycambia

Python wrapper for compact disc ripper log checking utility [cambia](https://github.com/arg274/cambia) (written in Rust). Use `pycambia` to parse and score CD rip logs from various rippers.

- [Installation](#installation)
- [Usage](#usage)
  - [parse_file](#cambiaparse_filefile_path)
  - [parse_content](#cambiaparse_contentcontent)
  - [supported_rippers](#cambiasupported_rippers)
  - [LogParser](#cambialogparser)
- [Return Format](#return-format)
- [Supported Rippers](#supported-rippers)
- [Development](#development)
- [License](#license)

## Installation

Install from PyPI:

```sh
pip install pycambia
```

Import in your Python code:

```py
import cambia
```

## Usage

### cambia.parse_file(file_path)

Parse a CD rip log file from disk.

**Parameters**:

- **file_path** _(str)_ – path to the log file to parse

**Returns**

- _(dict)_ – dictionary containing parsed data with success status and data/error information

**Examples:**

Parse an EAC log file:

```py
import cambia

result = cambia.parse_file("/path/to/eac.log")
if result["success"]:
    data = result["data"]
    print(f"Ripper: {data.get('ripper')}")
    print(f"Album: {data.get('album')}")
    print(f"Artist: {data.get('artist')}")
else:
    print(f"Error: {result['error']}")
```

### cambia.parse_content(content)

Parse log content from a string.

**Parameters**:

- **content** _(str)_ – log file content as string

**Returns**

- _(dict)_ – dictionary containing parsed data with success status and data/error information

**Examples:**

```py
import cambia

with open("/path/to/eac.log", "r", encoding="utf-8") as f:
    log_content = f.read()

result = cambia.parse_content(log_content)
if result["success"]:
    data = result["data"]
    print(f"Tracks: {len(data.get('tracks', []))}")
    print(f"Score: {data.get('score')}")
else:
    print(f"Parsing failed: {result['error']}")
```

### cambia.supported_rippers()

Get list of supported CD ripper log types.

**Returns**

- _(list[str])_ – list of supported CD ripper type names

**Examples:**

```py
import cambia

formats = cambia.supported_rippers()
print("Supported formats:", formats)
# Output: ['eac', 'xld', 'whipper', 'cueripper']
```

### cambia.LogParser

A high-level class interface for parsing CD ripping log files. Provides the same functionality as the module-level functions.

**Methods:**

#### LogParser.parse_file(file_path)

Static method equivalent to `cambia.parse_file()`.

#### LogParser.parse_content(content)

Static method equivalent to `cambia.parse_content()`.

#### LogParser.supported_rippers()

Static method equivalent to `cambia.supported_rippers()`.

**Examples:**

```py
import cambia

# Using the class interface
parser = cambia.LogParser()
result = parser.parse_file("/path/to/log.txt")

# Or using static methods
result = cambia.LogParser.parse_file("/path/to/log.txt")
```

## Return Format

All parsing functions return a dictionary with the following structure:

```py
{
    "success": bool,           # True if parsing succeeded, False otherwise
    "file_path": str,          # Path to the parsed file (only for parse_file)
    "data": dict | None,       # Parsed log data (if success=True)
    "error": str | None        # Error message (if success=False)
}
```

### Successful Parse Result

When `success` is `True`, the `data` field contains a comprehensive dictionary with parsed log information:

```py
{
    "evaluation_combined": [   # Scoring and evaluation results
        {
            "combined_score": str,     # Overall score (e.g., "100")
            "evaluations": [...],      # Detailed evaluation breakdown
            "evaluator": str           # Scoring system used (e.g., "OPS")
        }
    ],
    "id": [int, ...],         # Unique identifier array
    "parsed": {               # Main parsed log data
        "encoding": str,      # File encoding (e.g., "UTF-16LE")
        "parsed_logs": [      # Array of parsed log entries
            {
                "ripper": str,                    # Ripper software (e.g., "Exact Audio Copy")
                "ripper_version": str,            # Version (e.g., "1.6")
                "release_info": {                 # Album information
                    "artist": str,                # Album artist
                    "title": str                  # Album title
                },
                "drive": str,                     # CD drive model
                "read_mode": str,                 # Read mode (e.g., "Secure")
                "read_offset": int,               # Read offset value
                "accurate_stream": str,           # Accurate stream setting
                "defeat_audio_cache": str,        # Audio cache defeat setting
                "test_and_copy": str,             # Test & copy mode
                "checksum": {                     # Overall checksum info
                    "calculated": str,            # Calculated checksum
                    "log": str,                   # Log checksum
                    "integrity": str              # Match status
                },
                "toc": {                          # Table of contents data
                    "raw": {                      # Raw TOC information
                        "entries": [              # Track entries
                            {
                                "track": int,     # Track number
                                "start": float,   # Start time in seconds
                                "length": float,  # Track length in seconds
                                "start_sector": int,
                                "end_sector": int
                            }
                        ]
                    },
                    "freedb": {"hash": str, "url": str},
                    "accurip_tocid": {"hash": str, "url": str},
                    "ctdb_tocid": {"hash": str, "url": str},
                    "mbz": {"hash": str, "url": str}
                },
                "tracks": [                       # Individual track results
                    {
                        "num": int,               # Track number
                        "aborted": bool,          # Whether extraction was aborted
                        "extraction_speed": float, # Extraction speed multiplier
                        "peak_level": float,      # Peak audio level (0.0-1.0)
                        "filenames": [str],       # Output file paths
                        "test_and_copy": {        # Test & copy verification
                            "test_hash": str,     # Test pass hash
                            "copy_hash": str,     # Copy pass hash
                            "integrity": str      # Match status
                        },
                        "ar_info": [              # AccurateRip information
                            {
                                "status": str,    # Match status
                                "confidence": {   # Confidence data
                                    "matching": int,
                                    "total": int | None,
                                    "offset": str
                                },
                                "sign": str,      # AccurateRip signature
                                "version": int    # AccurateRip version
                            }
                        ],
                        "errors": dict,           # Any extraction errors
                        "pregap_length": float | None  # Pregap length if present
                    }
                ]
            }
        ]
    }
}
```

### Error Result

When `success` is `False`, the `error` field contains a description of what went wrong:

```py
{
    "success": False,
    "data": None,
    "error": "Unsupported log format or corrupted file"
}
```

**Example of handling results:**

```py
import cambia

result = cambia.parse_file("/path/to/eac.log")

if result["success"]:
    data = result["data"]
    
    # Get basic info
    parsed_log = data["parsed"]["parsed_logs"][0]
    print(f"Ripper: {parsed_log['ripper']} v{parsed_log['ripper_version']}")
    print(f"Artist: {parsed_log['release_info']['artist']}")
    print(f"Album: {parsed_log['release_info']['title']}")
    
    # Get score
    score = data["evaluation_combined"][0]["combined_score"]
    print(f"Score: {score}")
    
    # Handle tracks
    tracks = parsed_log["tracks"]
    print(f"Number of tracks: {len(tracks)}")
    
    for track in tracks:
        ar_status = track["ar_info"][0]["status"] if track["ar_info"] else "Unknown"
        print(f"Track {track['num']}: {ar_status} (Speed: {track['extraction_speed']}x)")
        
else:
    print(f"Failed to parse log: {result['error']}")
```

## Supported Rippers

pycambia supports parsing logs from the following CD ripping software:

| Ripper     | Status       | Description                                    |
| ---------- | ------------ | ---------------------------------------------- |
| EAC        | ✅ Stable     | Exact Audio Copy - Windows CD ripper          |
| XLD        | ✅ Stable     | X Lossless Decoder - macOS CD ripper          |
| whipper    | ✅ Stable     | Command-line CD ripper (successor to morituri) |
| CUERipper  | ⚠️ Experimental | Windows CD ripper                              |

**Examples for each ripper:**

EAC log parsing:
```py
import cambia

result = cambia.parse_file("/path/to/eac.log")
if result["success"]:
    data = result["data"]
    parsed_log = data["parsed"]["parsed_logs"][0]
    
    print(f"Ripper: {parsed_log['ripper']} v{parsed_log['ripper_version']}")
    print(f"Drive: {parsed_log['drive']}")
    print(f"Read mode: {parsed_log['read_mode']}")
    print(f"Score: {data['evaluation_combined'][0]['combined_score']}")
    
    # Check AccurateRip results
    for track in parsed_log["tracks"]:
        if track["ar_info"]:
            ar_status = track["ar_info"][0]["status"]
            confidence = track["ar_info"][0]["confidence"]["matching"]
            print(f"Track {track['num']}: {ar_status} (Confidence: {confidence})")
```

XLD log parsing:
```py
import cambia

result = cambia.parse_file("/path/to/xld.log")
if result["success"]:
    data = result["data"]
    parsed_log = data["parsed"]["parsed_logs"][0]
    
    print(f"Ripper: {parsed_log['ripper']}")
    print(f"Album: {parsed_log['release_info']['title']}")
    print(f"Artist: {parsed_log['release_info']['artist']}")
    print(f"Tracks: {len(parsed_log['tracks'])}")
    
    # Check extraction speeds
    speeds = [track["extraction_speed"] for track in parsed_log["tracks"]]
    avg_speed = sum(speeds) / len(speeds)
    print(f"Average extraction speed: {avg_speed:.1f}x")
```

whipper log parsing:
```py
import cambia

result = cambia.parse_file("/path/to/whipper.log")
if result["success"]:
    data = result["data"]
    parsed_log = data["parsed"]["parsed_logs"][0]
    
    print(f"Ripper: {parsed_log['ripper']}")
    print(f"Drive: {parsed_log['drive']}")
    print(f"Read offset: {parsed_log['read_offset']}")
    
    # Check for any aborted tracks
    aborted_tracks = [t for t in parsed_log["tracks"] if t["aborted"]]
    if aborted_tracks:
        print(f"Warning: {len(aborted_tracks)} tracks were aborted")
    
    # Check checksum integrity
    checksum_status = parsed_log["checksum"]["integrity"]
    print(f"Overall checksum: {checksum_status}")
```

## Development

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [Python 3.8+](https://www.python.org/downloads/)
1. Install [maturin](https://github.com/PyO3/maturin)
   ```sh
   pip install maturin
   ```
1. Clone this repository and navigate to it via command line
   ```sh
   git clone https://github.com/KyokoMiki/pycambia.git
   cd pycambia
   ```
1. Install development dependencies
   ```sh
   pip install -e ".[dev]"
   ```
1. Build the extension
   ```sh
   maturin develop
   ```
1. Run tests
   ```sh
   pytest
   ```
1. Format code
   ```sh
   ruff check .
   ruff format .
   ```

### Building for distribution

Build wheel:
```sh
maturin build --release
```

Build and publish to PyPI:
```sh
maturin publish
```

## License

MIT