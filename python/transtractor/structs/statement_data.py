"""Base data structure for recording extracted statement data for
subsequent processing in Python."""

import csv
from dataclasses import dataclass, field

from .transaction import Transaction


def _validate_fields(fields: list[str]) -> None:
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
        "opening_balance",
        "closing_balance",
        "key",
        "filename",
        "account_number",
    }
    for field_name in fields:
        if field_name not in valid_fields:
            raise ValueError(
                f"Invalid field: {field_name}. Valid fields are: {valid_fields}"
            )


@dataclass(repr=False, slots=True)
class StatementData:
    """Class representing bank statement data."""

    key: str = ""
    filename: str = ""
    account_number: str = ""
    start_date: int = 0
    opening_balance: float = 0.0
    closing_balance: float = 0.0
    transactions: list[Transaction] = field(default_factory=list)

    def __repr__(self) -> str:
        return (
            f"StatementData(key={self.key!r}, "
            f"filename={self.filename!r}, "
            f"account_number={self.account_number!r}, "
            f"start_date={self.start_date!r}, "
            f"opening_balance={self.opening_balance!r}, "
            f"closing_balance={self.closing_balance!r}, "
            f"transactions=[{len(self.transactions)} transactions])"
        )

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
            'start_date', 'opening_balance', 'closing_balance', 'key',
            'filename', 'account_number'.
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
        _validate_fields(list(fields))

        with open(file_path, mode="w", newline="", encoding="utf-8") as csvfile:
            writer = csv.writer(csvfile)
            # Write header
            writer.writerow(fields)
            # Write transaction data
            for transaction in self.transactions:
                row = []
                for field_name in fields:
                    if field_name in {
                        "key",
                        "filename",
                        "account_number",
                        "start_date",
                        "opening_balance",
                        "closing_balance",
                    }:
                        value = getattr(self, field_name, None)
                    else:
                        value = getattr(transaction, field_name, None)
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
        _validate_fields(list(fields))
        data_dict = {field_name: [] for field_name in fields}

        for transaction in self.transactions:
            for field_name in fields:
                if field_name in {
                    "key",
                    "filename",
                    "account_number",
                    "start_date",
                    "opening_balance",
                    "closing_balance",
                }:
                    value = getattr(self, field_name, None)
                else:
                    value = getattr(transaction, field_name, None)
                data_dict[field_name].append(value)

        return data_dict
