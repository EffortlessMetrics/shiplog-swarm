"""Normalization concerns."""


def normalize_rows(rows: list[dict]) -> list[dict]:
    """Trim and standardize manifest fields."""
    normalized: list[dict] = []
    for row in rows:
        normalized.append(
            {
                "tracking_id": str(row.get("tracking_id", "")).strip().upper(),
                "destination": str(row.get("destination", "")).strip(),
                "weight_kg": float(row.get("weight_kg", 0) or 0),
                "priority": str(row.get("priority", "standard")).strip().lower(),
            }
        )
    return normalized
