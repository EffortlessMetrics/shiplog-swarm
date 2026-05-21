"""Facade that orchestrates shiplog entry processing."""

from .submodules.parser import parse_raw_entry
from .submodules.normalizer import normalize_entry
from .submodules.validator import validate_required_fields


def process_shiplog_entry(raw_entry: str) -> dict:
    """Process a raw shiplog line into a validated, normalized record."""
    parsed = parse_raw_entry(raw_entry)
    validate_required_fields(parsed)
    return normalize_entry(parsed)
