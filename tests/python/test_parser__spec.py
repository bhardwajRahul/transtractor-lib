"""Tests for the Parser parse method."""

import tempfile
from pathlib import Path

import pytest
from transtractor import ParseError
from transtractor.parser import Parser


def test_spec_generates_correct_file():
    """Test that parsing test1.pdf generates a spec matching test1_spec.json."""
    parser = Parser()

    # Parse the test PDF
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))
    expected_spec = fixtures_dir / "test1_spec.json"

    # Generate spec in a temporary file
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".json", delete=False, newline=""
    ) as tmp_file:
        tmp_spec_path = tmp_file.name
        parser.spec(str(test_pdf), tmp_spec_path)

    try:
        # Read both spec files
        with open(tmp_spec_path, encoding="utf-8") as generated:
            generated_content = generated.read()

        with open(expected_spec, encoding="utf-8") as expected:
            expected_content = expected.read()

        # Compare content
        assert generated_content == expected_content, (
            "Spec output mismatch:\n"
            f"Generated length: {len(generated_content)}\n"
            f"Expected length: {len(expected_content)}"
        )
    finally:
        # Clean up temporary file
        Path(tmp_spec_path).unlink(missing_ok=True)


def test_spec_raises_parse_error_without_config():
    """Test that spec without loading a config raises ParseError."""
    parser = Parser()

    # Parse the test PDF without loading the config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"

    # Should raise ParseError since no config is loaded
    with pytest.raises(ParseError):
        parser.spec(str(test_pdf), "dummy_path.json")


def test_spec_raises_parse_error_with_misconfigured_config():
    """Test that spec with a misconfigured config raises ParseError."""
    parser = Parser()

    # Load the misconfigured config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"
    misconfigured_config = fixtures_dir / "test1_config_misconfigured.json"
    parser.load(str(misconfigured_config))

    # Should raise ParseError since the config is misconfigured
    with pytest.raises(ParseError):
        parser.spec(str(test_pdf), "dummy_path.json")
