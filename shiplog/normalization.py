"""Normalize and validate incoming log lines."""


def normalize_entries(raw_lines: list[str]) -> list[dict[str, object]]:
    records: list[dict[str, object]] = []

    for line in raw_lines:
        parts = [part.strip() for part in line.split(",")]
        if len(parts) != 3:
            continue

        ship_name, status, duration_text = parts
        if not ship_name:
            continue

        try:
            duration = float(duration_text)
        except ValueError:
            continue

        records.append(
            {
                "ship": ship_name.upper(),
                "status": status.lower(),
                "duration": duration,
            }
        )

    return records
