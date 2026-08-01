from pathlib import Path

import pytest
from transtractor import SpecError
from transtractor.parser import Parser


def test_validate_spec_with_valid_spec_file():
    parser = Parser()

    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    valid_spec = fixtures_dir / "test1_spec.json"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))

    # Should not raise any exception
    parser.validate_spec(str(valid_spec))


def test_validate_spec_with_invalid_spec_file():
    parser = Parser()

    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    invalid_spec = fixtures_dir / "test1_spec_invalid.json"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))

    # Should raise SpecError since the spec file is invalid
    with pytest.raises(SpecError):
        parser.validate_spec(str(invalid_spec))


def test_validate_spec_raises_spec_error_without_config():
    parser = Parser()

    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    valid_spec = fixtures_dir / "test1_spec.json"

    # Should raise SpecError since no config is loaded, not a ParseError
    with pytest.raises(SpecError):
        parser.validate_spec(str(valid_spec))
