"""Ship log processing utilities organized by SRP-focused submodules."""

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class VoyageRecord:
    vessel: str
    distance_nm: float
    fuel_tons: float


def parse_record(line: str) -> VoyageRecord:
    """Parse a CSV line into a VoyageRecord."""
    vessel, distance, fuel = [part.strip() for part in line.split(",")]
    return VoyageRecord(vessel=vessel, distance_nm=float(distance), fuel_tons=float(fuel))


def calculate_efficiency(record: VoyageRecord) -> float:
    """Calculate nautical miles per ton of fuel."""
    if record.fuel_tons <= 0:
        raise ValueError("fuel_tons must be positive")
    return record.distance_nm / record.fuel_tons


def summarize_efficiency(lines: Iterable[str]) -> dict[str, float]:
    """Original all-in-one behavior split into parse + compute + aggregate steps."""
    totals: dict[str, list[float]] = {}

    for line in lines:
        record = parse_record(line)
        efficiency = calculate_efficiency(record)
        totals.setdefault(record.vessel, []).append(efficiency)

    return {
        vessel: sum(values) / len(values)
        for vessel, values in totals.items()
    }
