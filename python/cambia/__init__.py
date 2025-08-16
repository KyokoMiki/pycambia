"""
Python wrapper for compact disc ripper log checking utility cambia
(https://github.com/arg274/cambia - written in Rust).
Use this module to parse and score CD rip logs from various rippers.
"""

from ._cambia import get_supported_rippers, parse_log_content, parse_log_file

# Version is automatically set by maturin from Cargo.toml
try:
    from ._cambia import __version__
except ImportError:
    # Fallback version if import fails
    __version__ = "unknown"

__all__ = ["parse_log_file", "parse_log_content", "get_supported_rippers", "LogParser"]


class LogParser:
    """
    A high-level interface for parsing CD ripping log files.
    """

    @staticmethod
    def parse_file(file_path: str) -> dict:
        """
        Parse a log file from disk.

        Args:
            file_path (str): Path to the log file

        Returns:
            dict: Dictionary with keys: 'success' (bool), 'data' (dict|None), 'error' (str|None)
        """
        return parse_log_file(file_path)

    @staticmethod
    def parse_content(content: str) -> dict:
        """
        Parse log content from a string.

        Args:
            content (str): Log file content as string

        Returns:
            dict: Dictionary with keys: 'success' (bool), 'data' (dict|None), 'error' (str|None)
        """
        return parse_log_content(content)

    @staticmethod
    def supported_rippers() -> list:
        """
        Get list of supported CD ripper log types.

        Returns:
            list: List of supported CD ripper type names
        """
        return get_supported_rippers()


# Convenience functions for direct use
def parse_file(file_path: str) -> dict:
    """
    Parse a log file from disk.

    Returns:
        dict: Dictionary with keys: 'success' (bool), 'data' (dict|None), 'error' (str|None)
    """
    return LogParser.parse_file(file_path)


def parse_content(content: str) -> dict:
    """
    Parse log content from a string.

    Returns:
        dict: Dictionary with keys: 'success' (bool), 'data' (dict|None), 'error' (str|None)
    """
    return LogParser.parse_content(content)


def supported_rippers() -> list:
    """Get list of supported CD ripper log types."""
    return LogParser.supported_rippers()
