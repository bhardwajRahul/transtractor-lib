"""Python wrapper for the Transtractor PDF bank statement parser."""

import warnings
from pathlib import Path
from typing import cast

from .structs.statement_data import StatementData
from .transtractor import LibParser
from .utils.testing import run_test_protocol


class Parser:
    """A PDF bank statement parser.

    This parser will be initialised with a set of default bank statement
    extraction configurations. When parsing a PDF, it will attempt to identify
    applicable configurations based on keywords extracted from the PDF. You
    can also load custom configurations from JSON files for additional statement
    formats.

    Example:
        parser = Parser()
        parser.load('custom_config.json')
        statement_data = parser.parse('statement.pdf')
        print(statement_data)
        statement_data.to_csv('output.csv')
    """

    def __init__(self):
        """Initialise the Parser with default database."""
        self._inner = LibParser()

    def parse(self, pdf_file_path: str) -> StatementData:
        """Parse the bank statement PDF and return a StatementData object.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises NoErrorFreeStatementData: Statement format recognised but could be
            processed without failing quality checks
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        sd: StatementData = cast(
            StatementData, self._inner.py_pdf_path_to_py_statement_data(pdf_file_path)
        )
        sd.set_filename(pdf_file_path)
        return sd

    def parse_layout(self, layout_file_path: str) -> StatementData:
        """Parse the bank statement layout string and return a StatementData object.

        :param layout_file_path: Path to the layout file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises NoErrorFreeStatementData: Statement format recognised but could be
            processed without failing quality checks
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        py_layout_str = open(layout_file_path, encoding="utf-8").read()
        sd: StatementData = cast(
            StatementData, self._inner.layout_py_str_py_statement_data(py_layout_str)
        )
        return sd

    def debug(self, pdf_file_path: str, output_file: str) -> str:
        """Write a summary of the statement data and quality checks for
        each statement extraction configuration applied.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output debug text file
        :return: The debug string written to the output file
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        result = self._inner.py_pdf_path_to_debug_py_str(pdf_file_path)
        with open(output_file, "w", encoding="utf-8") as fh:
            fh.write(result)
        return result

    def debug_layout(self, layout_file_path: str, output_file: str) -> str:
        """Write a summary of the statement data and quality checks for
        each statement extraction configuration applied.

        :param layout_file_path: Path to the layout file to be processed
        :param output_file: Path to the output debug text file
        :return: The debug string written to the output file
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        py_layout_str = open(layout_file_path, encoding="utf-8").read()
        result = self._inner.layout_py_str_to_debug_py_str(py_layout_str)
        with open(output_file, "w", encoding="utf-8") as fh:
            fh.write(result)
        return result

    def layout(self, pdf_file_path: str, output_file: str) -> str:
        """Extract, write and return a text layout representation of the PDF page.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: A string representing the text layout of the page
        """
        layout_str: str = self._inner.py_pdf_path_to_layout_py_str(pdf_file_path)
        with open(output_file, "w", encoding="utf-8") as fh:
            fh.write(layout_str)
        return layout_str

    def load(self, json_file_path: str) -> None:
        """Load a custom parsing configuration from a JSON file.

        Configurations loaded via this method will be registered in the
        internal database and will overwrite any existing configuration with the same
        key.

        :param json_file_path: Path to the JSON configuration file
        :return: None
        :raises ConfigLoadError: Configuration file is invalid or cannot be loaded

        See the docs for detailed instructions for creating custom
        configuration JSON files.
        """
        # Read the JSON file
        file_path = Path(json_file_path)
        json_content = file_path.read_text(encoding="utf-8")

        # Register the configuration via string to capture deprecation warnings
        self._inner.register_config_from_json_str(json_content)

        # Check for and emit any deprecation warnings
        deprecation_warnings = self._inner.get_deprecation_warnings()
        for warning_msg in deprecation_warnings:
            warnings.warn(
                f"Configuration from {json_file_path} uses deprecated field: "
                f"{warning_msg}",
                DeprecationWarning,
                stacklevel=2,
            )

    def test(
        self, pdf_dir: str, output_file: str = "", log_level: str = "INFO"
    ) -> None:
        """Try to parse all PDFs in a given directory and sub-directories
        using the current parser configuration database. Optionally outputs
        a CSV file summarising the test results.

        :param pdf_dir: Path to the directory containing PDF files to be tested
        :param output_file: Optional path to output CSV file for test results
        :param log_level: Logging level for test output (e.g., "INFO", "WARNING")
        :return: None

        Note: Set log_level to "WARNING" or higher to suppress terminal output.
        """
        run_test_protocol(pdf_dir, self, output_file, log_level)
