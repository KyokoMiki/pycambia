"""Integration tests for cambia package."""

from pathlib import Path

import cambia
import pytest


class TestBasicFunctionality:
    """Test basic cambia functionality."""

    def test_get_supported_rippers(self) -> None:
        """Test getting list of supported rippers."""
        rippers = cambia.get_supported_rippers()

        assert isinstance(rippers, list)
        assert len(rippers) > 0

        # All items should be Ripper enum members
        for ripper in rippers:
            assert isinstance(ripper, cambia.Ripper)

        # Check for known ripper types
        expected_rippers = [cambia.Ripper.EAC, cambia.Ripper.XLD, cambia.Ripper.Whipper]
        for expected in expected_rippers:
            assert expected in rippers

    def test_parse_empty_content(self) -> None:
        """Test that parsing empty content raises ValueError."""
        with pytest.raises(ValueError, match="Empty request body"):
            _ = cambia.parse_log_content("")

    def test_parse_invalid_content(self) -> None:
        """Test that parsing invalid content raises ValueError."""
        invalid_content = "This is not a valid log file content"
        with pytest.raises(ValueError, match="Unsupported file"):
            _ = cambia.parse_log_content(invalid_content)

    def test_parse_nonexistent_file(self) -> None:
        """Test that parsing nonexistent file raises OSError."""
        with pytest.raises(OSError, match="Could not read file"):
            _ = cambia.parse_log_file("nonexistent_file_12345.log")


class TestFileOperations:
    """Test file operations."""

    def test_parse_file_with_string_path(self, test_logs_dir: Path) -> None:
        """Test parsing with string path."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        # Should work with string path
        result = cambia.parse_log_file(str(eac_log))
        assert isinstance(result, cambia.CambiaResponse)

    def test_temporary_file_parsing(self, tmp_path: Path) -> None:
        """Test parsing temporary files."""
        test_file = tmp_path / "test.log"
        test_file.write_text("Invalid log content for testing", encoding="utf-8")

        # Invalid content should raise ValueError
        with pytest.raises(ValueError):
            _ = cambia.parse_log_file(test_file)

    def test_parse_content_with_string(self, test_logs_dir: Path) -> None:
        """Test parsing content read as string."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        # Read file as string
        content = eac_log.read_text(encoding="utf-8")
        result = cambia.parse_log_content(content)

        assert isinstance(result, cambia.CambiaResponse)
        assert len(result.parsed.parsed_logs) > 0

    def test_parse_content_with_bytes(self, test_logs_dir: Path) -> None:
        """Test parsing content read as bytes."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        # Read file as bytes
        content_bytes = eac_log.read_bytes()
        result = cambia.parse_log_content(content_bytes)

        assert isinstance(result, cambia.CambiaResponse)
        assert len(result.parsed.parsed_logs) > 0


class TestResponseStructure:
    """Test response data structure."""

    def test_response_structure(self, test_logs_dir: Path) -> None:
        """Test that response has correct structure."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        result = cambia.parse_log_file(eac_log)

        # Check CambiaResponse structure
        assert isinstance(result, cambia.CambiaResponse)
        assert isinstance(result.id, bytes)
        assert isinstance(result.parsed, cambia.ParsedLogCombined)
        assert isinstance(result.evaluation_combined, list)

    def test_parsed_log_structure(self, test_logs_dir: Path) -> None:
        """Test ParsedLogCombined structure."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        result = cambia.parse_log_file(eac_log)
        parsed = result.parsed

        assert isinstance(parsed.encoding, str)
        assert isinstance(parsed.parsed_logs, list)
        assert len(parsed.parsed_logs) > 0

    def test_parsed_log_entry_structure(self, test_logs_dir: Path) -> None:
        """Test ParsedLog entry structure."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        result = cambia.parse_log_file(eac_log)
        log_entry = result.parsed.parsed_logs[0]

        assert isinstance(log_entry, cambia.ParsedLog)
        assert isinstance(log_entry.ripper, cambia.Ripper)
        assert isinstance(log_entry.tracks, list)
        assert isinstance(log_entry.release_info, cambia.ReleaseInfo)
        assert isinstance(log_entry.checksum, cambia.Checksum)
        assert isinstance(log_entry.toc, cambia.Toc)

    def test_track_entry_structure(self, test_logs_dir: Path) -> None:
        """Test TrackEntry structure."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        result = cambia.parse_log_file(eac_log)
        assert len(result.parsed.parsed_logs) > 0
        log_entry = result.parsed.parsed_logs[0]

        assert len(log_entry.tracks) > 0
        track = log_entry.tracks[0]

        assert isinstance(track, cambia.TrackEntry)
        assert isinstance(track.num, int)
        assert isinstance(track.is_range, bool)
        assert isinstance(track.aborted, bool)
        assert isinstance(track.filenames, list)
        assert isinstance(track.test_and_copy, cambia.TestAndCopy)
        assert isinstance(track.errors, cambia.TrackError)
        assert isinstance(track.ar_info, list)

    def test_evaluation_structure(self, test_logs_dir: Path) -> None:
        """Test evaluation structure."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        result = cambia.parse_log_file(eac_log)

        assert len(result.evaluation_combined) > 0
        eval_combined = result.evaluation_combined[0]

        assert isinstance(eval_combined, cambia.EvaluationCombined)
        assert isinstance(eval_combined.evaluator, cambia.EvaluatorType)
        assert isinstance(eval_combined.combined_score, str)
        assert isinstance(eval_combined.evaluations, list)

    def test_evaluation_unit_structure(self, test_logs_dir: Path) -> None:
        """Test evaluation unit structure."""
        eac_log = test_logs_dir / "EAC" / "perf-hunid.log"

        if not eac_log.exists():
            pytest.skip("Test log file not found")

        result = cambia.parse_log_file(eac_log)
        assert len(result.evaluation_combined) > 0
        eval_combined = result.evaluation_combined[0]

        for evaluation in eval_combined.evaluations:
            assert isinstance(evaluation, cambia.Evaluation)
            assert isinstance(evaluation.score, str)

            for unit in evaluation.evaluation_units:
                assert isinstance(unit, cambia.EvaluationUnit)
                assert isinstance(unit.data, cambia.EvaluationUnitData)
                assert isinstance(unit.data.field, cambia.EvaluationUnitField)
                assert isinstance(unit.data.classification, cambia.EvaluationUnitClass)


class TestMultipleRippers:
    """Test parsing logs from different rippers."""

    @pytest.mark.parametrize(
        ("ripper_dir", "expected_ripper"),
        [
            ("EAC", cambia.Ripper.EAC),
            ("XLD", cambia.Ripper.XLD),
            ("whipper", cambia.Ripper.Whipper),
        ],
    )
    def test_ripper_detection(
        self, ripper_dir: str, expected_ripper: cambia.Ripper, test_logs_dir: Path
    ) -> None:
        """Test that ripper type is correctly detected.

        Args:
            ripper_dir: Directory name containing logs.
            expected_ripper: Expected ripper enum value.
            test_logs_dir: Path to test logs directory.
        """
        logs_dir = test_logs_dir / ripper_dir

        if not logs_dir.exists():
            pytest.skip(f"Logs directory not found: {logs_dir}")

        # Find first .log file
        log_files = list(logs_dir.glob("*.log"))
        if not log_files:
            pytest.skip(f"No log files found in {logs_dir}")

        result = cambia.parse_log_file(log_files[0])
        assert result.parsed.parsed_logs[0].ripper == expected_ripper


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_parse_content_with_unicode(self) -> None:
        """Test content with Unicode characters."""
        unicode_content = "Test content with émojis 🎵 and special chars: àáâãäå"
        with pytest.raises(ValueError):
            _ = cambia.parse_log_content(unicode_content)

    def test_parse_very_long_invalid_content(self) -> None:
        """Test very long invalid string."""
        long_content = "A" * 10000  # 10KB string
        with pytest.raises(ValueError):
            _ = cambia.parse_log_content(long_content)

    def test_parse_invalid_binary_content(self) -> None:
        """Test that invalid binary content raises ValueError."""
        binary_content = b"\x00\x01\x02\x03\x04\x05"
        # Invalid binary content should raise ValueError
        with pytest.raises(ValueError):
            _ = cambia.parse_log_content(binary_content)


class TestCombinedLogs:
    """Test combined log files."""

    def test_combined_log_detection(self, test_logs_dir: Path) -> None:
        """Test that combined logs are properly detected and parsed."""
        combined_log = test_logs_dir / "EAC" / "abort.log"

        if not combined_log.exists():
            pytest.skip("Combined log file not found")

        result = cambia.parse_log_file(combined_log)

        # Verify parsed_logs is a list
        assert isinstance(result.parsed.parsed_logs, list)
        # Verify that at least one log was parsed
        assert len(result.parsed.parsed_logs) > 0, (
            "Combined log should contain at least one parsed log entry"
        )
