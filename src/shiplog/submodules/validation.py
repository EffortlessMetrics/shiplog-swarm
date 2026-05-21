"""Validation concerns."""


def _is_valid_row(row: dict) -> bool:
    return bool(row["tracking_id"] and row["destination"] and row["weight_kg"] > 0)


def validate_rows(rows: list[dict]) -> tuple[list[dict], list[dict]]:
    """Split rows into valid and rejected buckets."""
    valid: list[dict] = []
    rejected: list[dict] = []
    for row in rows:
        (valid if _is_valid_row(row) else rejected).append(row)
    return valid, rejected
