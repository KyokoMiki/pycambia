#!/usr/bin/env python3
"""
pytest test file for testing pycambia package functionality
"""

import os
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
        rippers = cambia.supported_rippers()

        assert isinstance(rippers, list)
        assert len(rippers) > 0

        # Check for known ripper types
        expected_rippers = ["eac", "xld", "whipper"]
        for ripper in expected_rippers:
            assert ripper in rippers

    def test_parse_content_empty_string(self):
        """Test parsing empty string"""
        result = cambia.parse_content("")

        assert isinstance(result, dict)
        assert "success" in result
        assert result["success"] is False
        assert "error" in result
        assert isinstance(result["error"], str)

    def test_parse_content_invalid_content(self):
        """Test parsing invalid content"""
        invalid_content = "This is not a valid log file content"
        result = cambia.parse_content(invalid_content)

        assert isinstance(result, dict)
        assert result["success"] is False
        assert "error" in result
        assert isinstance(result["error"], str)

    def test_parse_file_nonexistent(self):
        """Test parsing nonexistent file"""
        result = cambia.parse_file("nonexistent_file.log")

        assert isinstance(result, dict)
        assert result["success"] is False
        assert "error" in result
        assert isinstance(result["error"], str)


class TestCambiaLogParser:
    """Test LogParser class"""

    def test_logparser_supported_rippers(self):
        """Test LogParser.supported_rippers() static method"""
        rippers = cambia.LogParser.supported_rippers()

        assert isinstance(rippers, list)
        assert len(rippers) > 0

    def test_logparser_parse_content_empty(self):
        """Test LogParser.parse_content() static method"""
        result = cambia.LogParser.parse_content("")

        assert isinstance(result, dict)
        assert result["success"] is False

    def test_logparser_parse_file_nonexistent(self):
        """Test LogParser.parse_file() static method"""
        result = cambia.LogParser.parse_file("nonexistent.log")

        assert isinstance(result, dict)
        assert result["success"] is False

    def test_logparser_instance_methods(self):
        """Test LogParser instance methods"""
        parser = cambia.LogParser()

        # Test that instance methods return same results as static methods
        static_rippers = cambia.LogParser.supported_rippers()
        instance_rippers = parser.supported_rippers()
        assert static_rippers == instance_rippers


class TestCambiaWithTestFiles:
    """Test using actual test files"""

    def test_parse_eac_log_file(self, test_files_dir):
        """Test parsing EAC log file"""
        eac_log_path = test_files_dir / "eac.log"

        if not eac_log_path.exists():
            pytest.skip("EAC test file does not exist")

        result = cambia.parse_file(str(eac_log_path))

        assert isinstance(result, dict)
        assert "success" in result

        # If parsing succeeds, check data structure
        if result["success"]:
            assert "data" in result
            assert result["data"] is not None
            assert isinstance(result["data"], dict)
        else:
            assert "error" in result

            # Check basic data structure
            data = result["data"]
            assert "parsed" in data
            assert "evaluation_combined" in data

            parsed = data["parsed"]
            assert "parsed_logs" in parsed
            assert isinstance(parsed["parsed_logs"], list)
            assert len(parsed["parsed_logs"]) > 0

            log_entry = parsed["parsed_logs"][0]
            assert "ripper" in log_entry
            assert "tracks" in log_entry
            assert isinstance(log_entry["tracks"], list)

    def test_parse_xld_log_file(self, test_files_dir):
        """Test parsing XLD log file"""
        xld_log_path = test_files_dir / "xld.log"

        if not xld_log_path.exists():
            pytest.skip("XLD test file does not exist")

        result = cambia.parse_file(str(xld_log_path))

        assert isinstance(result, dict)
        assert "success" in result

        # If parsing succeeds, check XLD-specific data
        if result["success"]:
            data = result["data"]
            log_entry = data["parsed"]["parsed_logs"][0]

            # XLD-specific checks
            assert "ripper" in log_entry
            if "ripper" in log_entry:
                assert (
                    "xld" in log_entry["ripper"].lower()
                    or "x lossless decoder" in log_entry["ripper"].lower()
                )

    def test_parse_eac_log_content(self, test_files_dir):
        """Test parsing EAC log content"""
        eac_log_path = test_files_dir / "eac.log"

        if not eac_log_path.exists():
            pytest.skip("EAC test file does not exist")

        with open(eac_log_path, encoding="utf-8", errors="ignore") as f:
            content = f.read()

        result = cambia.parse_content(content)

        assert isinstance(result, dict)
        assert "success" in result


class TestCambiaReturnFormat:
    """Test return format consistency"""

    def test_return_format_structure(self):
        """Test return format structure consistency"""
        # Test consistency of return format across different functions
        functions_to_test = [
            lambda: cambia.parse_content(""),
            lambda: cambia.parse_file("nonexistent.log"),
            lambda: cambia.LogParser.parse_content(""),
            lambda: cambia.LogParser.parse_file("nonexistent.log"),
        ]

        for func in functions_to_test:
            result = func()

            # Check basic structure
            assert isinstance(result, dict)
            assert "success" in result

            # Check types
            assert isinstance(result["success"], bool)

            # If failed, error should be present and be a string
            if not result["success"]:
                assert "error" in result
                assert isinstance(result["error"], str)
                assert len(result["error"]) > 0
            else:
                assert "data" in result

    def test_successful_parse_structure(self, test_files_dir):
        """Test data structure when parsing succeeds"""
        # Try to find a file that can be successfully parsed
        test_files = ["eac.log", "xld.log"]

        for filename in test_files:
            filepath = test_files_dir / filename
            if filepath.exists():
                result = cambia.parse_file(str(filepath))

                if result["success"]:
                    # Check data structure when parsing succeeds
                    assert "data" in result
                    assert result["data"] is not None
                    assert isinstance(result["data"], dict)

                    data = result["data"]

                    # Check required top-level keys
                    required_keys = ["parsed", "evaluation_combined"]
                    for key in required_keys:
                        assert key in data, f"Missing required key: {key}"

                    # Check parsed structure
                    parsed = data["parsed"]
                    assert "parsed_logs" in parsed
                    assert isinstance(parsed["parsed_logs"], list)

                    if len(parsed["parsed_logs"]) > 0:
                        log_entry = parsed["parsed_logs"][0]

                        # Check basic fields in log entry
                        expected_fields = ["ripper", "tracks"]
                        for field in expected_fields:
                            assert field in log_entry, (
                                f"Missing field in log entry: {field}"
                            )

                    # Check evaluation_combined structure
                    evaluation = data["evaluation_combined"]
                    assert isinstance(evaluation, list)

                    return  # One successful test is enough

        pytest.skip("No successfully parseable test files found")


class TestCambiaEdgeCases:
    """Test edge cases"""

    def test_parse_content_with_unicode(self):
        """Test content with Unicode characters"""
        unicode_content = "Test content with émojis 🎵 and special chars: àáâãäå"
        result = cambia.parse_content(unicode_content)

        assert isinstance(result, dict)
        assert "success" in result
        # Expected to fail, but should not crash
        assert result["success"] is False

    def test_parse_content_very_long_string(self):
        """Test very long string"""
        long_content = "A" * 10000  # 10KB string
        result = cambia.parse_content(long_content)

        assert isinstance(result, dict)
        assert result["success"] is False

    def test_parse_file_with_special_characters_in_path(self):
        """Test paths with special characters"""
        special_paths = [
            "file with spaces.log",
            "file-with-dashes.log",
            "file_with_underscores.log",
        ]

        for path in special_paths:
            result = cambia.parse_file(path)
            assert isinstance(result, dict)
            assert (
                result["success"] is False
            )  # File doesn't exist, but should not crash

    def test_temporary_file_parsing(self):
        """Test temporary file parsing"""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".log", delete=False) as tmp:
            tmp.write("Temporary log content for testing")
            tmp_path = tmp.name

        try:
            result = cambia.parse_file(tmp_path)
            assert isinstance(result, dict)
            assert "success" in result
        finally:
            os.unlink(tmp_path)


if __name__ == "__main__":
    # Allow running this file directly for testing
    pytest.main([__file__, "-v"])
