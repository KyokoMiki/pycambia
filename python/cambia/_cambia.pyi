"""Type stubs for the Rust extension module."""

import os
from datetime import timedelta
from enum import Enum

# ============= Enums =============

class Ripper(Enum):
    """CD ripper software type."""

    _value_: str

    EAC = ...
    XLD = ...
    Whipper = ...
    CueRipper = ...
    DBPA = ...
    CyanRip = ...
    EZCD = ...
    Morituri = ...
    Rip = ...
    FreAc = ...
    Other = ...

class MediaType(Enum):
    """Media type."""

    _value_: str

    Pressed = ...
    CDR = ...
    Other = ...
    Unknown = ...

class Quartet(Enum):
    """Four-state boolean value."""

    _value_: str

    TRUE = ...
    FALSE = ...
    UNKNOWN = ...
    UNSUPPORTED = ...

class ReadMode(Enum):
    """CD reading mode."""

    _value_: str

    Secure = ...
    Paranoid = ...
    Fast = ...
    Burst = ...
    Unknown = ...

class Gap(Enum):
    """Gap handling method."""

    _value_: str

    Append = ...
    AppendNoHtoa = ...
    AppendUndetected = ...
    Prepend = ...
    Discard = ...
    Unknown = ...
    Inapplicable = ...

class Integrity(Enum):
    """Data integrity status."""

    _value_: str

    Match = ...
    Mismatch = ...
    Unknown = ...

class AccurateRipStatus(Enum):
    """AccurateRip match status."""

    _value_: str

    Match = ...
    Mismatch = ...
    Offsetted = ...
    NotFound = ...
    Disabled = ...

class EvaluatorType(Enum):
    """Evaluation system type."""

    _value_: str

    Cambia = ...
    RED = ...
    OPS = ...

class EvaluationUnitField(Enum):
    """Field of evaluation unit."""

    _value_: str

    Encoding = ...
    RipperVersion = ...
    Drive = ...
    Ripper = ...
    Offset = ...
    Cache = ...
    TestAndCopy = ...
    Encoder = ...
    Checksum = ...
    MediaType = ...
    ReadMode = ...
    MaxRetryCount = ...
    AccurateStream = ...
    C2 = ...
    SilentSamples = ...
    NullSamples = ...
    Gap = ...
    Tag = ...
    Gain = ...
    RangeSplit = ...
    Samples = ...
    SilentBlocks = ...
    Normalization = ...
    Filename = ...
    ReadError = ...
    SkipError = ...
    JitterGenericError = ...
    JitterEdgeError = ...
    JitterAtomError = ...
    DriftError = ...
    DroppedError = ...
    DuplicatedError = ...
    InconsistentErrorSectors = ...
    DamagedSector = ...
    Abort = ...

class EvaluationUnitClass(Enum):
    """Class of evaluation unit."""

    _value_: str

    Critical = ...
    Bad = ...
    Neutral = ...
    Good = ...
    Perfect = ...

# ============= Classes =============

class TocEntry:
    """Represents a single TOC (Table of Contents) entry."""

    track: int
    start: timedelta
    length: timedelta
    start_sector: int
    end_sector: int

class TocHash:
    """Hash information for various disc ID services."""

    hash: str
    url: str

class TocRaw:
    """Raw TOC information."""

    entries: list[TocEntry]

class Toc:
    """Table of contents data with various disc IDs."""

    raw: TocRaw
    freedb: TocHash
    accurip_tocid: TocHash
    ctdb_tocid: TocHash
    mbz: TocHash
    gn: TocHash
    mcdi: TocHash

class Checksum:
    """Checksum information."""

    calculated: str
    log: str
    integrity: Integrity

class ReleaseInfo:
    """Album release information."""

    artist: str
    title: str

class AccurateRipConfidence:
    """AccurateRip confidence information."""

    matching: int | None
    total: int | None
    offset: str

class AccurateRipUnit:
    """AccurateRip information for a track."""

    status: AccurateRipStatus
    confidence: AccurateRipConfidence | None
    sign: str
    version: int | None

class TestAndCopy:
    """Test and copy hash verification."""

    test_hash: str
    copy_hash: str
    integrity: Integrity

class TrackErrorRange:
    """Range of track errors."""

    start: timedelta
    length: timedelta

class TrackErrorData:
    """Track error data."""

    count: int
    ranges: list[TrackErrorRange]

class TrackError:
    """Track error information."""

    read: TrackErrorData
    skip: TrackErrorData
    jitter_generic: TrackErrorData
    jitter_edge: TrackErrorData
    jitter_atom: TrackErrorData
    drift: TrackErrorData
    dropped: TrackErrorData
    duplicated: TrackErrorData
    damaged_sectors: TrackErrorData
    inconsistent_err_sectors: TrackErrorData
    missing_samples: TrackErrorData

class TrackEntry:
    """Individual track information."""

    num: int
    is_range: bool
    aborted: bool
    filenames: list[str]
    peak_level: float | None
    pregap_length: timedelta | None
    extraction_speed: float | None
    gain: float | None
    preemphasis: bool | None
    test_and_copy: TestAndCopy
    errors: TrackError
    ar_info: list[AccurateRipUnit]

class ParsedLog:
    """Parsed log data from a single ripper log."""

    ripper: Ripper
    ripper_version: str
    release_info: ReleaseInfo
    language: str
    read_offset: int | None
    combined_rw_offset: int | None
    drive: str
    media_type: MediaType
    accurate_stream: Quartet
    defeat_audio_cache: Quartet
    use_c2: Quartet
    overread: Quartet
    fill_silence: Quartet
    delete_silence: Quartet
    use_null_samples: Quartet
    test_and_copy: Quartet
    normalize: Quartet
    read_mode: ReadMode
    gap_handling: Gap
    checksum: Checksum
    toc: Toc
    tracks: list[TrackEntry]
    id3_enabled: Quartet
    audio_encoder: list[str]

class ParsedLogCombined:
    """Main parsed data container."""

    encoding: str
    parsed_logs: list[ParsedLog]

class EvaluationUnitScope:
    """Scope of evaluation unit."""

    name: str
    value: str

class EvaluationUnitData:
    """Evaluation unit data."""

    scope: EvaluationUnitScope
    field: EvaluationUnitField
    message: str
    classification: EvaluationUnitClass

class EvaluationUnit:
    """Single evaluation unit."""

    unit_score: str
    data: EvaluationUnitData

class Evaluation:
    """Individual evaluation with units."""

    score: str
    evaluation_units: list[EvaluationUnit]

class EvaluationCombined:
    """Combined evaluation results."""

    evaluator: EvaluatorType
    combined_score: str
    evaluations: list[Evaluation]

class CambiaResponse:
    """Main response data from Cambia."""

    id: bytes
    parsed: ParsedLogCombined
    evaluation_combined: list[EvaluationCombined]

# ============= Functions =============

def parse_log_file(path: str | os.PathLike[str]) -> CambiaResponse:
    """Parse a CD ripping log file and return the parsed data.

    Args:
        path: Path to the log file. Accepts a string or any os.PathLike
            object (e.g. pathlib.Path).

    Returns:
        Parsed log data.

    Raises:
        ValueError: If parsing fails.
    """
    ...

def parse_log_content(content: str | bytes) -> CambiaResponse:
    """Parse log content from a string or bytes.

    Args:
        content: Log file content as string or bytes.
            When bytes, the encoding is auto-detected by cambia-core.

    Returns:
        Parsed log data.

    Raises:
        ValueError: If parsing fails.
        TypeError: If content is not str or bytes.
    """
    ...

def get_supported_rippers() -> list[str]:
    """Get list of supported CD ripper log types.

    Returns:
        List of supported CD ripper type names.
    """
    ...

__version__: str
