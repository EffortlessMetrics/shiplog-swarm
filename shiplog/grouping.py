"""Grouping helpers for normalized ship log entries."""


def group_by_ship(entries: list[dict[str, object]]) -> dict[str, list[dict[str, object]]]:
    grouped: dict[str, list[dict[str, object]]] = {}

    for entry in entries:
        grouped.setdefault(entry["ship"], []).append(entry)

    return grouped
