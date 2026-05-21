"""Parsing concerns only."""


def parse_raw_entry(raw_entry: str) -> dict:
    """Parse a CSV-like shiplog line: vessel,port,eta,cargo_tons."""
    parts = [part.strip() for part in raw_entry.split(",")]
    if len(parts) != 4:
        raise ValueError("Expected 4 comma-separated values: vessel, port, eta, cargo_tons")

    vessel, port, eta, cargo_tons = parts
    return {
        "vessel": vessel,
        "port": port,
        "eta": eta,
        "cargo_tons": cargo_tons,
    }
