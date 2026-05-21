def normalize_rows(rows: list[dict]) -> list[dict]:
    """Normalize row data and discard incomplete rows."""
    normalized: list[dict] = []
    for row in rows:
        category = str(row.get("category", "")).strip().lower()
        amount_raw = row.get("amount")
        if not category or amount_raw is None:
            continue

        try:
            amount = float(amount_raw)
        except (TypeError, ValueError):
            continue

        normalized.append({"category": category, "amount": amount})

    return normalized
