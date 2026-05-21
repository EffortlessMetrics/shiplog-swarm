def totals_by_category(rows: list[dict]) -> dict[str, float]:
    """Aggregate normalized rows by category."""
    totals: dict[str, float] = {}
    for row in rows:
        category = row["category"]
        totals[category] = totals.get(category, 0.0) + row["amount"]
    return totals
