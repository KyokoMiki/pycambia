"""Type stubs for the Rust extension module."""

from typing import Any

def parse_log_file(path: str) -> dict[str, Any]:
    """
    Parse a CD ripping log file and return the parsed data.

    Args:
        path (str): Path to the log file

    Returns:
        dict: Dictionary containing 'success' (bool), 'data' (dict or None), and 'error' (str or None)
    """
    ...

def parse_log_content(content: str) -> dict[str, Any]:
    """
    Parse log content from a string.

    Args:
        content (str): Log file content as string

    Returns:
        dict: Dictionary containing 'success' (bool), 'data' (dict or None), and 'error' (str or None)
    """
    ...

def get_supported_rippers() -> list[str]:
    """
    Get list of supported CD ripper log types.

    Returns:
        list[str]: List of supported CD ripper type names
    """
    ...
