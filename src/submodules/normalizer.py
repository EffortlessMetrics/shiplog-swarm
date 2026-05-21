from __future__ import annotations


def normalize_rows(rows: list[dict]) -> list[dict]:
    """Normalize incoming records into a predictable shape."""
    normalized: list[dict] = []
    for row in rows:
        sku = str(row.get("sku", "")).strip().upper()
        qty = int(row.get("qty", 0) or 0)
        unit_price = float(row.get("unit_price", 0) or 0)
        normalized.append({"sku": sku, "qty": qty, "unit_price": unit_price})
    return normalized
