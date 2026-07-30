"""Stub file for transtractor Rust extension module."""

from .structs.statement_data import StatementData

class LibParser:
    """Parser for extracting statement data from text items."""

    def __init__(self) -> None:
        """Create a new LibParser instance."""

    def register_config_from_json(self, py_config_json_path: str) -> None:
        """
        Register JSON configuration file into the parser database.

        :param py_config_json_path: Path to the JSON configuration file
        :raises ConfigLoadError: If the configuration file cannot be loaded
        """

    def get_deprecation_warnings(self) -> list[str]:
        """
        Get deprecation warnings from the last loaded configuration.

        :return: List of deprecation warnings
        """

    def py_pdf_path_to_py_statement_data(self, py_pdf_path: str) -> StatementData:
        """
        Process a PDF file path from Python caller and return a Python StatementData
        object.

        :param py_pdf_path: Path to the PDF file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

    def py_pdf_path_to_layout(self, py_pdf_path: str, py_layout_path: str) -> None:
        """
        Process a PDF file into layout text str.

        :param py_pdf_path: Path to the PDF file
        :param py_layout_path: Path to the output layout text file
        """

    def py_pdf_path_to_debug(self, py_pdf_path: str, py_debug_path: str) -> None:
        """Process a PDF file path from Python caller and write debug information to a
        file.

        :param py_pdf_path: Path to the PDF file
        :param py_debug_path: Path to the output debug text file
        """

    def py_layout_path_to_py_statement_data(self, py_layout_path: str) -> StatementData:
        """
        Process a layout text file from Python caller and return statement data as a
        Python object of type StatementData.

        :param py_layout_path: Path to the layout text file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

    def py_layout_path_to_debug(self, py_layout_path: str, py_debug_path: str) -> None:
        """
        Process a layout text file from Python caller and write debug information to a
        text file.

        :param py_layout_path: Path to the layout text file
        :param py_debug_path: Path to the output debug text file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

    def py_pdf_path_to_spec(self, py_pdf_path: str, py_spec_path: str) -> None:
        """
        Process a PDF file path from Python caller and write a JSON spec string to a
        file.

        :param py_pdf_path: Path to the PDF file
        :param py_spec_path: Path to the output JSON spec file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

    def py_layout_path_to_spec(self, py_layout_path: str, py_spec_path: str) -> None:
        """
        Process a layout text file from Python caller and write a JSON spec string to a
        file.

        :param py_layout_path: Path to the layout text file
        :param py_spec_path: Path to the output JSON spec file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

class ParseError(Exception):
    """Raised when the content of a PDF file cannot be parsed correctly."""

class ConfigLoadError(Exception):
    """Raised when a configuration cannot be loaded."""
