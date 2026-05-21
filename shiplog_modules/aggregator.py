from typing import Iterable

from .metrics import calculate_efficiency
from .parser import parse_record


def summarize_efficiency(lines: Iterable[str]) -> dict[str, float]:
    """Aggregate average efficiency per vessel from CSV rows."""
    totals: dict[str, list[float]] = {}

    for line in lines:
        record = parse_record(line)
        efficiency = calculate_efficiency(record)
        totals.setdefault(record.vessel, []).append(efficiency)

    return {vessel: sum(values) / len(values) for vessel, values in totals.items()}
