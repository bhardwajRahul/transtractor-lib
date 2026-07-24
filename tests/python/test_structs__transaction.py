"""Tests for the Transaction struct."""

import os
import subprocess
import sys
from datetime import date as Date

from transtractor.structs.transaction import Transaction


def test_timestamp_converts_to_utc_date():
    """Test that a millisecond timestamp is converted to a date in UTC."""
    # 2025-01-01T00:00:00Z, as produced by the Rust core (midnight UTC)
    transaction = Transaction(
        date=1735689600000,
        date_index=0,
        description="Transaction 1",
        amount=50000.0,
        balance=100000.0,
    )

    assert transaction.date == Date(2025, 1, 1)


def test_timestamp_conversion_is_timezone_independent():
    """Test that the converted date does not depend on the local timezone.

    Runs the conversion in subprocesses pinned to timezones west and east
    of UTC, where a local-time interpretation of a midnight-UTC timestamp
    would shift the calendar date by a day.
    """
    script = (
        "from transtractor.structs.transaction import Transaction;"
        "t = Transaction(1735689600000, 0, 'Transaction 1', 50000.0, 100000.0);"
        "print(t.date.isoformat())"
    )

    for tz in ("America/New_York", "Australia/Sydney", "UTC"):
        env = {**os.environ, "TZ": tz}
        result = subprocess.run(
            [sys.executable, "-c", script],
            env=env,
            capture_output=True,
            text=True,
            check=True,
        )
        assert result.stdout.strip() == "2025-01-01", (
            f"Date shifted in timezone {tz}: got {result.stdout.strip()}"
        )
