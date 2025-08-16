"""Type stubs for the Rust extension module."""

from typing import Any

def parse_log_file(path: str) -> dict[str, Any]:
    """
    Parse a CD ripping log file and return the parsed data.

    Args:
        path: Path to the log file

    Returns:
        Dictionary with keys: 'success' (bool), 'data' (dict|None), 'error' (str|None)
    """
    ...

def parse_log_content(content: str) -> dict[str, Any]:
    """
    Parse log content from a string.

    Args:
        content: Log file content as string

    Returns:
        Dictionary with keys: 'success' (bool), 'data' (dict|None), 'error' (str|None)
    """
    ...

def get_supported_rippers() -> list[str]:
    """
    Get list of supported CD ripper log types.

    Returns:
        List of supported CD ripper type names
    """
    ...
