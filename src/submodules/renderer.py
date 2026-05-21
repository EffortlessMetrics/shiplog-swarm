from __future__ import annotations


def render_report(summary: dict) -> str:
    """Render aggregate summary into a deterministic text report."""
    lines = [
        "Order Report",
        f"Total Quantity: {summary['total_qty']}",
        f"Total Revenue: ${summary['total_revenue']:.2f}",
        "SKU Breakdown:",
    ]

    for sku, qty in sorted(summary["sku_breakdown"].items()):
        lines.append(f"- {sku}: {qty}")

    return "\n".join(lines)
