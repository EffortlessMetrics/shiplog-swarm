"""Normalization concerns only."""

from datetime import datetime


def normalize_entry(entry: dict) -> dict:
    """Normalize types and casing for downstream use."""
    normalized_eta = datetime.fromisoformat(entry["eta"]).date().isoformat()
    return {
        "vessel": entry["vessel"].title(),
        "port": entry["port"].upper(),
        "eta": normalized_eta,
        "cargo_tons": float(entry["cargo_tons"]),
    }
