from __future__ import annotations


def summarize_orders(rows: list[dict]) -> dict:
    """Convert normalized rows into aggregate metrics."""
    total_qty = 0
    total_revenue = 0.0
    sku_breakdown: dict[str, int] = {}

    for row in rows:
        sku = row["sku"]
        qty = row["qty"]
        total_qty += qty
        total_revenue += qty * row["unit_price"]
        sku_breakdown[sku] = sku_breakdown.get(sku, 0) + qty

    return {
        "total_qty": total_qty,
        "total_revenue": round(total_revenue, 2),
        "sku_breakdown": sku_breakdown,
    }
