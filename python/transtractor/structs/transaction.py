"""Python implementation of transaction data for further processing in Python."""

from dataclasses import dataclass
from datetime import date as Date
from datetime import datetime, timezone


@dataclass(eq=False)
class Transaction:
    """Class representing a bank transaction."""

    date: Date
    date_index: int
    description: str
    amount: float
    balance: float
    account_number: str

    def __init__(
        self,
        date: Date | int,
        date_index: int,
        description: str,
        amount: float,
        balance: float,
        account_number: str = "",
    ):
        """Initialize a Transaction.

        :param date: Either a date object or milliseconds since epoch (int)
        :param date_index: Transaction index for the day
        :param description: Transaction description
        :param amount: Transaction amount (will be rounded to 2 decimal places)
        :param balance: Account balance (will be rounded to 2 decimal places)
        :param account_number: Account number associated with the transaction
        """
        if isinstance(date, int):
            # Convert milliseconds since epoch to date
            # Timestamps from the Rust core are midnight UTC; convert in UTC
            # so the calendar date is identical in every local timezone.
            self.date = datetime.fromtimestamp(date / 1000.0, tz=timezone.utc).date()
        else:
            self.date = date
        self.date_index = date_index
        self.description = description
        self.amount = round(amount, 2)
        self.balance = round(balance, 2)
        self.account_number = account_number

    def _identity(self) -> tuple[Date, int, float, str]:
        return self.date, self.date_index, self.amount, self.account_number

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Transaction):
            return NotImplemented
        return self._identity() == other._identity()

    def __hash__(self) -> int:
        return hash(self._identity())
