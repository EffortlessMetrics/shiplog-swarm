"""Metric aggregation concerns."""


def compute_metrics(valid_rows: list[dict], rejected_rows: list[dict]) -> dict:
    """Compute totals and derived values used in reporting."""
    total_weight = sum(row["weight_kg"] for row in valid_rows)
    return {
        "received": len(valid_rows) + len(rejected_rows),
        "accepted": len(valid_rows),
        "rejected": len(rejected_rows),
        "total_weight_kg": round(total_weight, 2),
    }
