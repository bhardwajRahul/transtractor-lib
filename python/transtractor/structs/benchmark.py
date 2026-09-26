"""Stage timings in microseconds produced while parsing a statement."""

from dataclasses import dataclass


@dataclass(slots=True)
class Benchmark:
    total: int = 0
    pdf_extractor: int = 0
    tokeniser: int = 0
    typer: int = 0
    parsers: int = 0
    parsers_account_number_parser_prime: int = 0
    parsers_account_number_parser_parse: int = 0
    parsers_start_date_parser_prime: int = 0
    parsers_start_date_parser_parse: int = 0
    parsers_opening_balance_parser_prime: int = 0
    parsers_opening_balance_parser_parse: int = 0
    parsers_closing_balance_parser_prime: int = 0
    parsers_closing_balance_parser_parse: int = 0
    parsers_transaction_parser_start_prime: int = 0
    parsers_transaction_parser_parse: int = 0
    parsers_transaction_parser_stop_prime: int = 0
    fixers: int = 0
    checkers: int = 0
