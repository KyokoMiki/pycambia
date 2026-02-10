"""
Python wrapper for compact disc ripper log checking utility cambia
(https://github.com/arg274/cambia - written in Rust).
Use this module to parse and score CD rip logs from various rippers.
"""

from ._cambia import (
    AccurateRipConfidence,
    AccurateRipStatus,
    AccurateRipUnit,
    CambiaResponse,
    Checksum,
    Evaluation,
    EvaluationCombined,
    EvaluationUnit,
    EvaluationUnitClass,
    EvaluationUnitData,
    EvaluationUnitField,
    EvaluationUnitScope,
    EvaluatorType,
    Gap,
    Integrity,
    MediaType,
    ParsedLog,
    ParsedLogCombined,
    Quartet,
    ReadMode,
    ReleaseInfo,
    Ripper,
    TestAndCopy,
    Toc,
    TocEntry,
    TocHash,
    TocRaw,
    TrackEntry,
    TrackError,
    TrackErrorData,
    TrackErrorRange,
    get_supported_rippers,
    parse_log_content,
    parse_log_file,
)

# Version is automatically set by maturin from Cargo.toml
try:
    from ._cambia import __version__
except ImportError:
    # Fallback version if import fails
    __version__ = "unknown"

__all__ = [
    "__version__",
    "parse_log_file",
    "parse_log_content",
    "get_supported_rippers",
    # Enums
    "Ripper",
    "MediaType",
    "Quartet",
    "ReadMode",
    "Gap",
    "Integrity",
    "AccurateRipStatus",
    "EvaluatorType",
    # Data classes
    "CambiaResponse",
    "ParsedLogCombined",
    "ParsedLog",
    "ReleaseInfo",
    "Checksum",
    "Toc",
    "TocHash",
    "TocRaw",
    "TocEntry",
    "TrackEntry",
    "TrackError",
    "TrackErrorData",
    "TrackErrorRange",
    "TestAndCopy",
    "AccurateRipUnit",
    "AccurateRipConfidence",
    "EvaluationCombined",
    "Evaluation",
    "EvaluationUnit",
    "EvaluationUnitData",
    "EvaluationUnitField",
    "EvaluationUnitScope",
    "EvaluationUnitClass",
]
