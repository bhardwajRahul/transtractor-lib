"""Tests for Parser.load() method."""

import warnings
from pathlib import Path

import pytest
from transtractor.parser import Parser
from transtractor.transtractor import ConfigLoadError


def test_load_raises_config_load_error_with_invalid_config():
    """Test that loading an invalid config file raises ConfigLoadError."""
    parser = Parser()

    # Try to load an invalid config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    invalid_config = fixtures_dir / "test1_config_invalid.json"

    # Should raise ConfigLoadError since the config has an invalid country code
    with pytest.raises(ConfigLoadError):
        parser.load(str(invalid_config))


def test_load_emits_deprecation_warning_for_deprecated_fields():
    """Test that loading a config with deprecated fields emits a DeprecationWarning."""
    parser = Parser()

    # Load config with deprecated fields
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    deprecated_config = fixtures_dir / "test1_config_deprecated_fields.json"

    # Should emit a DeprecationWarning for fix_text_order field
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        parser.load(str(deprecated_config))

        # Verify that a DeprecationWarning was issued
        assert len(w) == 1
        assert issubclass(w[0].category, DeprecationWarning)
        assert "fix_text_order" in str(w[0].message)
        assert "deprecated since v0.10.0" in str(w[0].message)
