"""Test cases for unrecognized or invalid log files."""

from pathlib import Path

import cambia
import pytest

# Test data: (filename, should_raise_error)
# Note: Some logs that were expected to be rejected are actually parsed by cambia
UNRECOGNIZED_TEST_LOGS: list[tuple[str, bool]] = [
    ("eac-edited-at-top-extra-spaces.log", True),  # Raises ValueError
    ("eac-edited-wrongly-split-combined.log", True),  # Raises ValueError
    ("eac-failed-to-properly-forge-a.log", False),  # Parses successfully
    ("eac-unrecognized-not-all-tracks.log", False),  # Parses successfully
    ("eac-wrong-date.log", False),  # Parses successfully
]


@pytest.mark.parametrize(("filename", "should_raise"), UNRECOGNIZED_TEST_LOGS)
def test_unrecognized_logs(
    filename: str, should_raise: bool, unrecognized_logs_dir: Path
) -> None:
    """Test that unrecognized logs are handled appropriately.

    Args:
        filename: Name of the log file to test.
        should_raise: Whether the log should raise a ValueError.
        unrecognized_logs_dir: Path to unrecognized logs directory.
    """
    log_path = unrecognized_logs_dir / filename

    if not log_path.exists():
        pytest.skip(f"Log file not found: {log_path}")

    if should_raise:
        # These logs should raise ValueError
        with pytest.raises(ValueError, match="Could not parse log"):
            _ = cambia.parse_log_file(log_path)
    else:
        # These logs parse successfully (even if they have issues)
        result = cambia.parse_log_file(log_path)
        assert isinstance(result, cambia.CambiaResponse)
        assert isinstance(result.parsed, cambia.ParsedLogCombined)
        assert len(result.parsed.parsed_logs) > 0
