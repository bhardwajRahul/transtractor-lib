"""Tests for the Parser layout generation functionality."""

import tempfile
from pathlib import Path

from transtractor.parser import Parser


def test_layout_generates_correct_output_default_settings():
    """Test that layout for test1.pdf with default settings matches test1_layout.txt."""
    parser = Parser()

    # Generate layout for the test PDF with default settings
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_pdf = fixtures_dir / "test1.pdf"
    expected_layout = fixtures_dir / "test1_layout.txt"

    # Generate layout output in a temporary file
    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp_file:
        tmp_layout_path = tmp_file.name

    try:
        parser.layout(str(test_pdf), tmp_layout_path)

        # Read both layout files
        with open(tmp_layout_path, encoding="utf-8") as generated:
            generated_content = generated.read()

        with open(expected_layout, encoding="utf-8") as expected:
            expected_content = expected.read()

        # Compare content
        assert generated_content == expected_content, (
            "Layout output mismatch:\n"
            f"Generated length: {len(generated_content)}\n"
            f"Expected length: {len(expected_content)}"
        )
    finally:
        # Clean up temporary file
        Path(tmp_layout_path).unlink(missing_ok=True)
