"""Pytest configuration and shared fixtures for cambia tests."""

from pathlib import Path

import pytest


@pytest.fixture(scope="session")
def test_logs_dir() -> Path:
    """Return the test logs directory path.

    Returns:
        Path object pointing to the logs directory.
    """
    return Path(__file__).parent / "logs"


@pytest.fixture(scope="session")
def eac_logs_dir(test_logs_dir: Path) -> Path:
    """Return the EAC logs directory path.

    Args:
        test_logs_dir: The base test logs directory.

    Returns:
        Path object pointing to the EAC logs directory.
    """
    return test_logs_dir / "EAC"


@pytest.fixture(scope="session")
def eac95_logs_dir(test_logs_dir: Path) -> Path:
    """Return the EAC95 logs directory path.

    Args:
        test_logs_dir: The base test logs directory.

    Returns:
        Path object pointing to the EAC95 logs directory.
    """
    return test_logs_dir / "EAC95"


@pytest.fixture(scope="session")
def xld_logs_dir(test_logs_dir: Path) -> Path:
    """Return the XLD logs directory path.

    Args:
        test_logs_dir: The base test logs directory.

    Returns:
        Path object pointing to the XLD logs directory.
    """
    return test_logs_dir / "XLD"


@pytest.fixture(scope="session")
def whipper_logs_dir(test_logs_dir: Path) -> Path:
    """Return the whipper logs directory path.

    Args:
        test_logs_dir: The base test logs directory.

    Returns:
        Path object pointing to the whipper logs directory.
    """
    return test_logs_dir / "whipper"


@pytest.fixture(scope="session")
def unrecognized_logs_dir(test_logs_dir: Path) -> Path:
    """Return the unrecognized logs directory path.

    Args:
        test_logs_dir: The base test logs directory.

    Returns:
        Path object pointing to the unrecognized logs directory.
    """
    return test_logs_dir / "unrecognized"
