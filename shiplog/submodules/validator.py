"""Validation concerns only."""

REQUIRED_FIELDS = ("vessel", "port", "eta", "cargo_tons")


def validate_required_fields(entry: dict) -> None:
    """Validate presence and basic value constraints for required fields."""
    missing = [name for name in REQUIRED_FIELDS if not entry.get(name)]
    if missing:
        raise ValueError(f"Missing required fields: {', '.join(missing)}")

    try:
        cargo = float(entry["cargo_tons"])
    except ValueError as exc:
        raise ValueError("cargo_tons must be numeric") from exc

    if cargo < 0:
        raise ValueError("cargo_tons cannot be negative")
