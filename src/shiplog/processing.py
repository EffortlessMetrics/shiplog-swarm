"""Shipment processing orchestration.

Refactored into DRY/SRP helpers split by concern.
"""

from .submodules.normalization import normalize_rows
from .submodules.validation import validate_rows
from .submodules.metrics import compute_metrics
from .submodules.report import build_report


def process_shipment_manifest(raw_rows: list[dict]) -> dict:
    """Normalize, validate, measure, and report on shipment manifest rows."""
    normalized_rows = normalize_rows(raw_rows)
    valid_rows, rejected_rows = validate_rows(normalized_rows)
    metrics = compute_metrics(valid_rows, rejected_rows)
    return build_report(valid_rows, rejected_rows, metrics)
