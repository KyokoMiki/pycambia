"""
pytest test file for testing pycambia package functionality
"""

import tempfile
from pathlib import Path

import cambia
import pytest


@pytest.fixture
def test_files_dir():
    """Return test files directory path"""
    return Path(__file__).parent


class TestCambiaBasicFunctions:
    """Test basic functionality of cambia module"""

    def test_supported_rippers(self):
        """Test getting supported ripper list"""
        rippers = cambia.get_supported_rippers()

        assert isinstance(rippers, list)
        assert len(rippers) > 0

        # Check for known ripper types
        expected_rippers = ["EAC", "XLD", "whipper"]
        for ripper in expected_rippers:
            assert ripper in rippers

    def test_parse_content_empty_string(self):
        """Test parsing empty string raises ValueError"""
        with pytest.raises(ValueError, match="Empty request body"):
            cambia.parse_log_content("")

    def test_parse_content_invalid_content(self):
        """Test parsing invalid content raises ValueError"""
        invalid_content = "This is not a valid log file content"
        with pytest.raises(ValueError, match="Unsupported file"):
            cambia.parse_log_content(invalid_content)

    def test_parse_file_nonexistent(self):
        """Test parsing nonexistent file raises OSError"""
        with pytest.raises(OSError, match="Could not read file"):
            cambia.parse_log_file("nonexistent_file.log")


class TestCambiaWithTestFiles:
    """Test using actual test files"""

    def test_parse_eac_log_file(self, test_files_dir):
        """Test parsing EAC log file"""
        eac_log_path = test_files_dir / "eac.log"

        if not eac_log_path.exists():
            pytest.skip("EAC test file does not exist")

        result = cambia.parse_log_file(str(eac_log_path))

        assert isinstance(result, cambia.CambiaResponse)
        assert isinstance(result.id, bytes)
        assert isinstance(result.parsed, cambia.ParsedLogCombined)
        assert isinstance(result.evaluation_combined, list)

        # Check parsed structure
        parsed = result.parsed
        assert isinstance(parsed.parsed_logs, list)
        assert len(parsed.parsed_logs) > 0

        log_entry = parsed.parsed_logs[0]
        assert isinstance(log_entry, cambia.ParsedLog)
        assert isinstance(log_entry.ripper, cambia.Ripper)
        assert isinstance(log_entry.tracks, list)

    def test_parse_xld_log_file(self, test_files_dir):
        """Test parsing XLD log file"""
        xld_log_path = test_files_dir / "xld.log"

        if not xld_log_path.exists():
            pytest.skip("XLD test file does not exist")

        result = cambia.parse_log_file(str(xld_log_path))

        assert isinstance(result, cambia.CambiaResponse)

        # Check parsed structure
        log_entry = result.parsed.parsed_logs[0]
        assert isinstance(log_entry.ripper, cambia.Ripper)
        assert log_entry.ripper == cambia.Ripper.XLD

    def test_parse_eac_log_content(self, test_files_dir):
        """Test parsing EAC log content via parse_content (EAC uses UTF-16)"""
        eac_log_path = test_files_dir / "eac.log"

        if not eac_log_path.exists():
            pytest.skip("EAC test file does not exist")

        # Read as bytes and decode UTF-16 to test parse_content with a string
        raw_bytes = eac_log_path.read_bytes()
        decoded_text = raw_bytes.decode("utf-16")
        result = cambia.parse_log_content(decoded_text)
        assert isinstance(result, cambia.CambiaResponse)


class TestCambiaReturnFormat:
    """Test return format consistency"""

    def test_error_raises_valueerror(self):
        """Test that parsing errors raise appropriate exceptions"""
        with pytest.raises(ValueError):
            cambia.parse_log_content("")

        with pytest.raises(OSError):
            cambia.parse_log_file("nonexistent.log")

    def test_successful_parse_structure(self, test_files_dir):
        """Test data structure when parsing succeeds"""
        test_files = ["eac.log", "xld.log"]
        any_succeeded = False

        for filename in test_files:
            filepath = test_files_dir / filename
            if filepath.exists():
                result = cambia.parse_log_file(str(filepath))
                any_succeeded = True

                # Check CambiaResponse structure
                assert isinstance(result, cambia.CambiaResponse)
                assert isinstance(result.id, bytes)
                assert isinstance(result.parsed, cambia.ParsedLogCombined)
                assert isinstance(result.evaluation_combined, list)

                # Check ParsedLogCombined structure
                parsed = result.parsed
                assert isinstance(parsed.encoding, str)
                assert isinstance(parsed.parsed_logs, list)

                if len(parsed.parsed_logs) > 0:
                    log_entry = parsed.parsed_logs[0]
                    assert isinstance(log_entry, cambia.ParsedLog)
                    assert isinstance(log_entry.ripper, cambia.Ripper)
                    assert isinstance(log_entry.tracks, list)
                    assert isinstance(log_entry.release_info, cambia.ReleaseInfo)
                    assert isinstance(log_entry.checksum, cambia.Checksum)
                    assert isinstance(log_entry.toc, cambia.Toc)

                # Check evaluation structure
                for eval_combined in result.evaluation_combined:
                    assert isinstance(eval_combined, cambia.EvaluationCombined)
                    assert isinstance(eval_combined.evaluator, cambia.EvaluatorType)
                    assert isinstance(eval_combined.combined_score, str)
                    assert isinstance(eval_combined.evaluations, list)

        if not any_succeeded:
            pytest.skip("No successfully parseable test files found")


class TestCambiaEdgeCases:
    """Test edge cases"""

    def test_parse_content_with_unicode(self):
        """Test content with Unicode characters raises ValueError"""
        unicode_content = "Test content with émojis 🎵 and special chars: àáâãäå"
        with pytest.raises(ValueError):
            cambia.parse_log_content(unicode_content)

    def test_parse_content_very_long_string(self):
        """Test very long string raises ValueError"""
        long_content = "A" * 10000  # 10KB string
        with pytest.raises(ValueError):
            cambia.parse_log_content(long_content)

    def test_temporary_file_parsing(self):
        """Test temporary file parsing"""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".log", delete=False) as tmp:
            tmp.write("Temporary log content for testing")
            tmp_path = tmp.name

        try:
            # Invalid content should raise ValueError
            with pytest.raises(ValueError):
                cambia.parse_log_file(tmp_path)
        finally:
            Path(tmp_path).unlink()


class TestCambiaTrackDetails:
    """Test detailed track information from parsed logs"""

    def test_track_entry_fields(self, test_files_dir):
        """Test TrackEntry fields are properly populated"""
        eac_log_path = test_files_dir / "eac.log"

        if not eac_log_path.exists():
            pytest.skip("EAC test file does not exist")

        result = cambia.parse_log_file(str(eac_log_path))
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

    def test_evaluation_details(self, test_files_dir):
        """Test evaluation details from parsed logs"""
        eac_log_path = test_files_dir / "eac.log"

        if not eac_log_path.exists():
            pytest.skip("EAC test file does not exist")

        result = cambia.parse_log_file(str(eac_log_path))

        if len(result.evaluation_combined) > 0:
            eval_combined = result.evaluation_combined[0]
            assert isinstance(eval_combined.evaluator, cambia.EvaluatorType)
            assert isinstance(eval_combined.combined_score, str)

            for evaluation in eval_combined.evaluations:
                assert isinstance(evaluation, cambia.Evaluation)
                assert isinstance(evaluation.score, str)

                for unit in evaluation.evaluation_units:
                    assert isinstance(unit, cambia.EvaluationUnit)
                    assert isinstance(unit.data, cambia.EvaluationUnitData)
                    assert isinstance(unit.data.field, cambia.EvaluationUnitField)
                    assert isinstance(
                        unit.data.classification,
                        cambia.EvaluationUnitClass,
                    )


if __name__ == "__main__":
    # Allow running this file directly for testing
    pytest.main([__file__, "-v"])
