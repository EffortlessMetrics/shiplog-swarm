"""Metric calculations for grouped ship log entries."""


def compute_ship_metrics(grouped: dict[str, list[dict[str, object]]]) -> dict[str, dict[str, float]]:
    summary: dict[str, dict[str, float]] = {}

    for ship, entries in grouped.items():
        total_duration = sum(float(entry["duration"]) for entry in entries)
        active_duration = sum(
            float(entry["duration"]) for entry in entries if entry["status"] == "active"
        )
        event_count = float(len(entries))

        summary[ship] = {
            "total_duration": total_duration,
            "active_ratio": (active_duration / total_duration) if total_duration else 0.0,
            "event_count": event_count,
        }

    return summary
