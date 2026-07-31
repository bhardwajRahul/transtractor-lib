"""Base data structure for recording extracted statement data for
subsequent processing in Python."""

import csv

from .transaction import Transaction


def validate_fields(fields: list[str]) -> None:
    """Validate that the provided fields are valid Transaction attributes.

    :param fields: List of field names to validate
    :type fields: list[str]
    :raises ValueError: If any field is not a valid
        Transaction or StatementData attribute
    """
    valid_fields = {
        "date",
        "date_index",
        "description",
        "amount",
        "balance",
        "start_date",
        "start_date_year",
        "opening_balance",
        "closing_balance",
        "key",
        "filename",
        "account_number",
    }
    for field in fields:
        if field not in valid_fields:
            raise ValueError(
                f"Invalid field: {field}. Valid fields are: {valid_fields}"
            )


class StatementData:
    """Class representing bank statement data."""

    def __init__(
        self,
        key: str,
        account_number: str,
        start_date: int,
        start_date_year: int,
        opening_balance: float,
        closing_balance: float,
        transactions: list[Transaction],
    ):
        """Initialize StatementData with validated attributes.

        :param key: Unique identifier for the statement
        :type key: str
        :param account_number: Account number associated with the statement
        :type account_number: str
        :param start_date: Statement start date in milliseconds since epoch
        :type start_date: int
        :param start_date_year: Calendar year corresponding to start_date
        :type start_date_year: int
        :param opening_balance: Opening balance for the statement
        :type opening_balance: float
        :param closing_balance: Closing balance for the statement
        :type closing_balance: float
        :param transactions: List of transactions in the statement
        :type transactions: list[Transaction]
        """
        self._key = ""
        self._filename = ""
        self._account_number = ""
        self._start_date = 0
        self._start_date_year = 0
        self._opening_balance = 0.0
        self._closing_balance = 0.0
        self._transactions = []

        # Use setters to enforce types
        self.set_key(key)
        self.set_account_number(account_number)
        self.set_start_date(start_date)
        self.set_start_date_year(start_date_year)
        self.set_opening_balance(opening_balance)
        self.set_closing_balance(closing_balance)
        self.set_transactions(transactions)

    def __repr__(self) -> str:
        return (
            f"StatementData(key={self._key!r}, "
            f"filename={self._filename!r}, "
            f"account_number={self._account_number!r}, "
            f"start_date={self._start_date!r}, "
            f"start_date_year={self._start_date_year!r}, "
            f"opening_balance={self._opening_balance!r}, "
            f"closing_balance={self._closing_balance!r}, "
            f"transactions=[{len(self._transactions)} transactions])"
        )

    @property
    def key(self) -> str:
        """Get the statement key."""
        return self._key

    @property
    def filename(self) -> str:
        """Get the filename."""
        return self._filename

    @property
    def account_number(self) -> str:
        """Get the account number."""
        return self._account_number

    @property
    def start_date(self) -> int:
        """Get the statement start date in milliseconds since epoch."""
        return self._start_date

    @property
    def start_date_year(self) -> int:
        """Get the statement start-date year."""
        return self._start_date_year

    @property
    def opening_balance(self) -> float:
        """Get the opening balance."""
        return self._opening_balance

    @property
    def closing_balance(self) -> float:
        """Get the closing balance."""
        return self._closing_balance

    @property
    def transactions(self) -> list[Transaction]:
        """Get the list of transactions."""
        return self._transactions

    def set_key(self, key: str) -> None:
        """Set the key for the statement data.

        :param key: Unique identifier for the statement
        :type key: str
        :raises TypeError: If key is not a string
        """
        if not isinstance(key, str):
            raise TypeError(f"key must be a string, got {type(key).__name__}")
        self._key = key

    def set_filename(self, filename: str) -> None:
        """Set the filename for the statement data.

        :param filename: Filename for the statement
        :type filename: str
        :raises TypeError: If filename is not a string
        """
        if not isinstance(filename, str):
            raise TypeError(f"filename must be a string, got {type(filename).__name__}")
        self._filename = filename

    def set_account_number(self, account_number: str) -> None:
        """Set the account number for the statement data.

        :param account_number: Account number associated with the statement
        :type account_number: str
        :raises TypeError: If account_number is not a string
        """
        if not isinstance(account_number, str):
            raise TypeError(
                f"account_number must be a string, got {type(account_number).__name__}"
            )
        self._account_number = account_number

    def set_start_date(self, start_date: int) -> None:
        """Set the statement start date in milliseconds since epoch."""
        if not isinstance(start_date, int):
            raise TypeError(
                f"start_date must be an int, got {type(start_date).__name__}"
            )
        self._start_date = start_date

    def set_start_date_year(self, start_date_year: int) -> None:
        """Set the year derived from the statement start date."""
        if not isinstance(start_date_year, int):
            raise TypeError(
                f"start_date_year must be an int, got {type(start_date_year).__name__}"
            )
        self._start_date_year = start_date_year

    def set_opening_balance(self, opening_balance: float) -> None:
        """Set the opening balance for the statement."""
        if not isinstance(opening_balance, int | float):
            raise TypeError(
                f"opening_balance must be numeric, got {type(opening_balance).__name__}"
            )
        self._opening_balance = round(float(opening_balance), 2)

    def set_closing_balance(self, closing_balance: float) -> None:
        """Set the closing balance for the statement."""
        if not isinstance(closing_balance, int | float):
            raise TypeError(
                f"closing_balance must be numeric, got {type(closing_balance).__name__}"
            )
        self._closing_balance = round(float(closing_balance), 2)

    def set_transactions(self, transactions: list[Transaction]) -> None:
        """Set the transactions for the statement data.

        :param transactions: List of transactions
        :type transactions: list[Transaction]
        :raises TypeError: Transactions not a list or contain non-Transaction items
        """
        if not isinstance(transactions, list):
            raise TypeError(
                f"transactions must be a list, got {type(transactions).__name__}"
            )

        for i, transaction in enumerate(transactions):
            if not isinstance(transaction, Transaction):
                raise TypeError(
                    f"transactions[{i}] must be a Transaction instance, "
                    f"got {type(transaction).__name__}"
                )

        self._transactions = transactions

    def to_csv(
        self,
        file_path: str,
        fields: tuple[str, ...] | list[str] = (
            "date",
            "description",
            "amount",
            "balance",
        ),
    ) -> None:
        """Export the statement data to a CSV file.

        :param file_path: Path to the output CSV file
        :type file_path: str
        :param fields: Fields to include in the CSV. Defaults to
            ('date', 'description', 'amount', 'balance'). Valid fields are:
            'date', 'date_index', 'description', 'amount', 'balance',
            'start_date', 'start_date_year', 'opening_balance', 'closing_balance',
            'key', 'filename', 'account_number'.
        :type fields: Union[tuple[str, ...], list[str]]

        Example usage::

            # Export with default fields
            statement_data.to_csv('transactions.csv')

            # Export with all available fields using list (or tuple)
            statement_data.to_csv(
                'full_export.csv',
                fields=['date', 'date_index', 'description', 'amount',
                        'balance', 'key', 'filename', 'account_number']
            )
        """
        # Validate fields
        validate_fields(list(fields))

        with open(file_path, mode="w", newline="", encoding="utf-8") as csvfile:
            writer = csv.writer(csvfile)
            # Write header
            writer.writerow(fields)
            # Write transaction data
            for transaction in self._transactions:
                row = []
                for field in fields:
                    if field in {
                        "key",
                        "filename",
                        "account_number",
                        "start_date",
                        "start_date_year",
                        "opening_balance",
                        "closing_balance",
                    }:
                        value = getattr(self, f"_{field}", None)
                    else:
                        value = getattr(transaction, field, None)
                    row.append(value)
                writer.writerow(row)

    def to_pandas_dict(
        self,
        fields: tuple[str, ...] | list[str] = (
            "date",
            "description",
            "amount",
            "balance",
        ),
    ) -> dict[str, list]:
        """Convert the statement data to a dictionary suitable for pandas DataFrame.

        :param fields: Fields to include in the dictionary. Defaults to
            ('date', 'description', 'amount', 'balance').
        :return: Dictionary with keys as field names and values as lists of field values
        :rtype: dict[str, list]

        Example usage::

            # Default fields
            data_dict = statement_data.to_pandas_dict()
            df = pd.DataFrame(data_dict)

            # Custom fields with list (tuple also supported)
            data_dict = statement_data.to_pandas_dict(
                fields=['date', 'description', 'amount', 'balance', 'key']
            )
            df = pd.DataFrame(data_dict)
        """
        validate_fields(list(fields))
        data_dict = {field: [] for field in fields}

        for transaction in self._transactions:
            for field in fields:
                if field in {
                    "key",
                    "filename",
                    "account_number",
                    "start_date",
                    "start_date_year",
                    "opening_balance",
                    "closing_balance",
                }:
                    value = getattr(self, f"_{field}", None)
                else:
                    value = getattr(transaction, field, None)
                data_dict[field].append(value)

        return data_dict
