"""High-level orchestration for ship log summarization."""

from .normalization import normalize_entries
from .grouping import group_by_ship
from .metrics import compute_ship_metrics


def summarize_ship_logs(raw_lines: list[str]) -> dict[str, dict[str, float]]:
    """Convert raw CSV-like lines into summary metrics by ship.

    Expected format for each input line:
      SHIP_NAME,STATUS,DURATION_HOURS
    """
    entries = normalize_entries(raw_lines)
    grouped = group_by_ship(entries)
    return compute_ship_metrics(grouped)
