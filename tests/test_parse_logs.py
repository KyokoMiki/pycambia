"""Unified test cases for all ripper log parsing and evaluation."""

from pathlib import Path

import cambia
import pytest

# Test data structure: (subdir, filename, ripper, expected_score, expected_deductions)
# subdir: subdirectory under tests/logs/
# filename: log file name
# ripper: expected Ripper enum value
# expected_score: expected total score as string
# expected_deductions: dict mapping deduction message to unit score
RIPPER_TEST_CASES: list[tuple[str, str, cambia.Ripper, str, dict[str, str]]] = [
    # EAC logs
    ("EAC", "1.3-good.log", cambia.Ripper.EAC, "100", {}),
    (
        "EAC",
        "abort.log",
        cambia.Ripper.EAC,
        "100",
        {
            "Copy aborted": "100",
            "Could not verify filename or file extension": "1",
        },
    ),
    (
        "EAC",
        "bad-htoa.log",
        cambia.Ripper.EAC,
        "49",
        {
            "Could not verify gap handling": "10",
            "Could not verify id3 tag setting": "1",
            "Range rip detected": "30",
            "Test and copy was not used": "10",
        },
    ),
    (
        "EAC",
        "bad-russian-099.log",
        cambia.Ripper.EAC,
        "70",
        {
            "Test and copy was not used": "10",
            "Could not verify read offset": "1",
            "Combined read/write offset cannot be verified": "4",
            "Null samples should be used in CRC calculations": "5",
            '"Defeat audio cache" should be Yes/true': "10",
        },
    ),
    (
        "EAC",
        "badcombo.log",
        cambia.Ripper.EAC,
        "90",
        {
            "Copy aborted": "100",
            "Could not verify filename or file extension": "1",
            "Incorrect gap handling": "10",
        },
    ),
    ("EAC", "burst.log", cambia.Ripper.EAC, "80", {"Rip mode not secure": "20"}),
    ("EAC", "data-track.log", cambia.Ripper.EAC, "100", {}),
    ("EAC", "eac-99-good.log", cambia.Ripper.EAC, "100", {}),
    (
        "EAC",
        "fast.log",
        cambia.Ripper.EAC,
        "-11",
        {
            "Could not verify gap handling": "10",
            "Could not verify id3 tag setting": "1",
            "Test and copy was not used": "10",
            "Range rip detected": "30",
            "Rip mode not secure": "20",
            (
                "Rip was not done in Secure mode, and T+C was not used - "
                "as a result, we cannot verify the authenticity of the rip"
            ): "40",
        },
    ),
    (
        "EAC",
        "hella-aborted.log",
        cambia.Ripper.EAC,
        "69",
        {
            '"Defeat audio cache" should be Yes/true': "10",
            "Could not verify null samples": "0",
            "Incorrect gap handling": "10",
            "Copy aborted": "100",
            "Could not verify id3 tag setting": "1",
            "Test and copy was not used": "10",
        },
    ),
    (
        "EAC",
        "htoa-not-ripped-twice.log",
        cambia.Ripper.EAC,
        "50",
        {
            "Could not verify gap handling": "10",
            "Range rip detected": "30",
            "Test and copy was not used": "10",
        },
    ),
    ("EAC", "inconsistent-accuraterip.log", cambia.Ripper.EAC, "100", {}),
    (
        "EAC",
        "mac-roman-charset.log",
        cambia.Ripper.EAC,
        "74",
        {
            "Could not verify id3 tag setting": "1",
            '"Defeat audio cache" should be Yes/true': "10",
            "Null samples should be used in CRC calculations": "5",
            "Incorrect gap handling": "10",
        },
    ),
    ("EAC", "negative-offset.log", cambia.Ripper.EAC, "100", {}),
    (
        "EAC",
        "perf-hunid.log",
        cambia.Ripper.EAC,
        "100",
        {"The drive was not found in the database": "0"},
    ),
    (
        "EAC",
        "range-rip.log",
        cambia.Ripper.EAC,
        "60",
        {"Range rip detected": "30", "Could not verify gap handling": "10"},
    ),
    (
        "EAC",
        "russian-range-rip-ar-issue.log",
        cambia.Ripper.EAC,
        "50",
        {
            "Could not verify gap handling": "10",
            "Range rip detected": "30",
            "Test and copy was not used": "10",
        },
    ),
    (
        "EAC",
        "russian1.log",
        cambia.Ripper.EAC,
        "60",
        {"Could not verify gap handling": "10", "Range rip detected": "30"},
    ),
    (
        "EAC",
        "russian2.log",
        cambia.Ripper.EAC,
        "60",
        {"Range rip detected": "30", "Could not verify gap handling": "10"},
    ),
    (
        "EAC",
        "russian3.log",
        cambia.Ripper.EAC,
        "60",
        {"Could not verify gap handling": "10", "Range rip detected": "30"},
    ),
    (
        "EAC",
        "russian4.log",
        cambia.Ripper.EAC,
        "60",
        {"Could not verify gap handling": "10", "Range rip detected": "30"},
    ),
    (
        "EAC",
        "shitty.log",
        cambia.Ripper.EAC,
        "-50",
        {"Suspicious position(s) found": "20", "CRC mismatch": "30"},
    ),
    (
        "EAC",
        "spanish-099.log",
        cambia.Ripper.EAC,
        "70",
        {
            '"Defeat audio cache" should be Yes/true': "10",
            "Test and copy was not used": "10",
            "C2 pointers were used": "10",
        },
    ),
    (
        "EAC",
        "spanish-log-extra-colons.log",
        cambia.Ripper.EAC,
        "8",
        {
            '"Defeat audio cache" should be Yes/true': "10",
            "Rip mode not secure": "20",
            "Could not verify read mode": "1",
            "Could not verify read offset": "1",
            "Test and copy was not used": "10",
            "Incorrect gap handling": "10",
            (
                "Rip was not done in Secure mode, and T+C was not used - "
                "as a result, we cannot verify the authenticity of the rip"
            ): "40",
        },
    ),
    (
        "EAC",
        "swedish-timing-problems.log",
        cambia.Ripper.EAC,
        "-50",
        {
            "Rip was not done in Secure mode, and experienced CRC mismatches": "20",
            "Rip mode not secure": "20",
            "Timing problem(s) found": "20",
            "CRC mismatch": "30",
        },
    ),
    # EAC95 logs
    (
        "EAC95",
        "had-decoding-issues.log",
        cambia.Ripper.EAC,
        "59",
        {
            "EAC version older than 0.99": "30",
            "Could not verify id3 tag setting": "1",
            "Could not verify null samples": "0",
            "Could not verify gap handling": "10",
        },
    ),
    (
        "EAC95",
        "also-bad.log",
        cambia.Ripper.EAC,
        "44",
        {
            "Could not verify gap handling": "10",
            "EAC version older than 0.99": "30",
            "Could not verify id3 tag setting": "1",
            "Could not verify read offset": "1",
            "Combined read/write offset cannot be verified": "4",
            "Could not verify null samples": "0",
            "Test and copy was not used": "10",
        },
    ),
    (
        "EAC95",
        "bad-settings.log",
        cambia.Ripper.EAC,
        "44",
        {
            "Could not verify read offset": "1",
            "C2 pointers were used": "10",
            "EAC version older than 0.99": "30",
            "Could not verify null samples": "0",
            "Could not verify gap handling": "10",
            "Could not verify id3 tag setting": "1",
            "Combined read/write offset cannot be verified": "4",
        },
    ),
    (
        "EAC95",
        "read-mode.log",
        cambia.Ripper.EAC,
        "49",
        {
            "EAC version older than 0.99": "30",
            "Could not verify id3 tag setting": "1",
            "Could not verify null samples": "0",
            "Could not verify gap handling": "10",
            '"Defeat audio cache" should be Yes/true': "10",
        },
    ),
    (
        "EAC95",
        "burst.log",
        cambia.Ripper.EAC,
        "-16",
        {
            "Test and copy was not used": "10",
            "Could not verify id3 tag setting": "1",
            "Rip mode not secure": "20",
            "Could not verify gap handling": "10",
            "EAC version older than 0.99": "30",
            "Incorrect read offset for drive": "5",
            "Could not verify null samples": "0",
            (
                "Rip was not done in Secure mode, and T+C was not used - "
                "as a result, we cannot verify the authenticity of the rip"
            ): "40",
        },
    ),
    # XLD logs
    (
        "XLD",
        "cdparanoia.log",
        cambia.Ripper.XLD,
        "80",
        {
            "Could not verify gap handling": "10",
            "C2 pointers were used": "10",
            "Not a pressed cd": "0",
        },
    ),
    ("XLD", "crc-mismatch.log", cambia.Ripper.XLD, "70", {"CRC mismatch": "30"}),
    ("XLD", "htoa.log", cambia.Ripper.XLD, "100", {"Not a pressed cd": "0"}),
    (
        "XLD",
        "ripping-error.log",
        cambia.Ripper.XLD,
        "60",
        {"CRC mismatch": "30", "Damaged sectors": "10"},
    ),
    (
        "XLD",
        "range-vbox.log",
        cambia.Ripper.XLD,
        "99",
        {"Could not verify filename or file extension": "1"},
    ),
    ("XLD", "bad-chardet-no-checksum.log", cambia.Ripper.XLD, "100", {}),
    (
        "XLD",
        "cdr-multi-filename.log",
        cambia.Ripper.XLD,
        "100",
        {"Not a pressed cd": "0"},
    ),
    (
        "XLD",
        "xld-cdp.log",
        cambia.Ripper.XLD,
        "100",
        {"Not a pressed cd": "0"},
    ),
    ("XLD", "100-percent-new.log", cambia.Ripper.XLD, "100", {}),
    # whipper logs
    ("whipper", "whipper-good.log", cambia.Ripper.Whipper, "100", {}),
    (
        "whipper",
        "whipper-with-errors.log",
        cambia.Ripper.Whipper,
        "0",
        {"Logs must be produced by whipper 0.7.3+": "100"},
    ),
]


def extract_deduction_details(
    response: cambia.CambiaResponse,
) -> tuple[str, dict[str, str]]:
    """Extract total score and deduction details from evaluation results.

    Args:
        response: The parsed cambia response.

    Returns:
        Tuple of (total_score, deductions_dict) where deductions_dict maps
        message to unit_score.
    """
    assert response.evaluation_combined, (
        "evaluation_combined is empty, cannot extract total_score"
    )
    total_score = response.evaluation_combined[0].combined_score
    deductions: dict[str, str] = {}

    for eval_combined in response.evaluation_combined:
        for evaluation in eval_combined.evaluations:
            for unit in evaluation.evaluation_units:
                if hasattr(unit.data, "message") and unit.data.message:
                    deductions[unit.data.message] = unit.unit_score

    return total_score, deductions


@pytest.mark.parametrize(
    ("subdir", "filename", "ripper", "expected_score", "expected_deductions"),
    RIPPER_TEST_CASES,
)
def test_parse_log(
    subdir: str,
    filename: str,
    ripper: cambia.Ripper,
    expected_score: str,
    expected_deductions: dict[str, str],
    test_logs_dir: Path,
) -> None:
    """Test log parsing and scoring.

    Verifies that:
    1. Log file can be parsed successfully
    2. Ripper type is correctly identified
    3. Total score matches expected value
    4. All deductions match expected values

    Args:
        subdir: Subdirectory under tests/logs/ containing the log file.
        filename: Name of the log file to test.
        ripper: Expected ripper enum value.
        expected_score: Expected total score as string.
        expected_deductions: Dict mapping deduction message to unit score.
        test_logs_dir: Path to test logs directory.
    """
    log_path = test_logs_dir / subdir / filename

    if not log_path.exists():
        pytest.skip(f"Log file not found: {log_path}")

    # Parse log file
    result = cambia.parse_log_file(log_path)

    # Verify parsing succeeded
    assert isinstance(result, cambia.CambiaResponse)
    assert isinstance(result.parsed, cambia.ParsedLogCombined)
    assert len(result.parsed.parsed_logs) > 0

    # Verify ripper type
    for parsed_log in result.parsed.parsed_logs:
        assert parsed_log.ripper == ripper

    # Extract and verify scoring
    actual_score, actual_deductions = extract_deduction_details(result)

    deductions_str = "\n".join(
        f"  - {msg}: {score}" for msg, score in actual_deductions.items()
    )

    assert actual_score == expected_score, (
        f"Total score mismatch: expected {expected_score}, got {actual_score}\n"
        f"Actual deductions:\n{deductions_str}"
    )

    assert actual_deductions == expected_deductions, (
        f"Deductions mismatch:\n"
        f"Expected: {expected_deductions}\n"
        f"Actual:   {actual_deductions}"
    )
