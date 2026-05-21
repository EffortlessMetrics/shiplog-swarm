"""Refactor demo: break one function into DRY/SRP subfunctions."""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime
from typing import Iterable


@dataclass(frozen=True)
class ShipmentRecord:
    shipment_id: str
    route: str
    status: str
    weight_kg: float
    updated_at: str


@dataclass(frozen=True)
class ShipmentSummary:
    total_shipments: int
    delivered_shipments: int
    delayed_shipments: int
    avg_weight_kg: float
    latest_update: str | None
    route_counts: dict[str, int]


def summarize_shipments(records: Iterable[ShipmentRecord]) -> ShipmentSummary:
    """Summarize shipment records with single-purpose helpers.

    This function used to be a monolith; now it orchestrates small SRP helpers.
    """

    normalized = _normalize_records(records)
    counts = _count_statuses(normalized)
    avg_weight = _average_weight(normalized)
    latest = _latest_update(normalized)
    routes = _count_routes(normalized)

    return ShipmentSummary(
        total_shipments=len(normalized),
        delivered_shipments=counts["delivered"],
        delayed_shipments=counts["delayed"],
        avg_weight_kg=avg_weight,
        latest_update=latest,
        route_counts=routes,
    )


def _normalize_records(records: Iterable[ShipmentRecord]) -> list[ShipmentRecord]:
    """Clean record fields once to avoid repeated sanitation logic (DRY)."""
    out: list[ShipmentRecord] = []
    for record in records:
        out.append(
            ShipmentRecord(
                shipment_id=record.shipment_id.strip(),
                route=record.route.strip().upper(),
                status=record.status.strip().lower(),
                weight_kg=max(record.weight_kg, 0.0),
                updated_at=record.updated_at.strip(),
            )
        )
    return out


def _count_statuses(records: Iterable[ShipmentRecord]) -> dict[str, int]:
    """Count delivered and delayed statuses only."""
    counts = {"delivered": 0, "delayed": 0}
    for record in records:
        if record.status == "delivered":
            counts["delivered"] += 1
        elif record.status == "delayed":
            counts["delayed"] += 1
    return counts


def _average_weight(records: list[ShipmentRecord]) -> float:
    """Compute average shipment weight."""
    if not records:
        return 0.0
    return round(sum(r.weight_kg for r in records) / len(records), 2)


def _latest_update(records: Iterable[ShipmentRecord]) -> str | None:
    """Find the latest update timestamp."""
    latest: datetime | None = None
    for record in records:
        parsed = datetime.fromisoformat(record.updated_at)
        if latest is None or parsed > latest:
            latest = parsed
    return latest.isoformat() if latest else None


def _count_routes(records: Iterable[ShipmentRecord]) -> dict[str, int]:
    """Count shipments per route."""
    route_counter: dict[str, int] = defaultdict(int)
    for record in records:
        route_counter[record.route] += 1
    return dict(route_counter)
