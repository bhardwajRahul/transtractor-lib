"""Stub file for transtractor Rust extension module."""

from .structs.statement_data import StatementData

class LibParser:
    """Parser for extracting statement data from text items."""

    def __init__(self) -> None:
        """Create a new LibParser instance."""

    def register_config_from_json_str(self, py_json_str: str) -> None:
        """
        Register JSON configuration string into the parser database.

        :param py_json_str: JSON string containing the configuration
        :raises ConfigLoadError: If the configuration cannot be loaded
        """

    def register_config_from_file(self, py_file_path: str) -> None:
        """
        Register JSON configuration file into the parser database.

        :param py_file_path: Path to the JSON configuration file
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

    def py_pdf_path_to_layout_py_str(self, py_pdf_path: str) -> str:
        """
        Process a PDF file into layout text str.

        :param py_pdf_path: Path to the PDF file
        """

    def py_pdf_path_to_debug_py_str(self, py_pdf_path: str) -> str:
        """Process a PDF file path from Python caller and return debug information as a
        string.

        :param py_pdf_path: Path to the PDF file
        """

    def layout_py_str_py_statement_data(self, py_layout_str: str) -> StatementData:
        """
        Process a layout string and return statement data as a Python object of type
        StatementData.

        :param py_layout_str: Layout string content from text file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

    def layout_py_str_to_debug_py_str(self, py_layout_str: str) -> str:
        """
        Process a layout string and return debug information as a string.

        :param py_layout_str: Layout string content from text file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

    def py_pdf_path_to_spec_py_str(self, py_pdf_path: str) -> str:
        """
        Process a PDF file path from Python caller and return a JSON spec string.

        :param py_pdf_path: Path to the PDF file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """

class ParseError(Exception):
    """Raised when the content of a PDF file cannot be parsed correctly."""

class ConfigLoadError(Exception):
    """Raised when a configuration cannot be loaded."""
