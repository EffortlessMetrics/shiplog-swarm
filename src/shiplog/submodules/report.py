"""Reporting concerns."""


def build_report(valid_rows: list[dict], rejected_rows: list[dict], metrics: dict) -> dict:
    """Build output document for downstream systems."""
    return {
        "metrics": metrics,
        "accepted_tracking_ids": [row["tracking_id"] for row in valid_rows],
        "rejected_tracking_ids": [row["tracking_id"] for row in rejected_rows],
    }
