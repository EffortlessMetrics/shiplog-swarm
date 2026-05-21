from .normalize import normalize_rows
from .aggregate import totals_by_category
from .render import render_markdown


def build_sales_report(rows: list[dict]) -> str:
    """Create a markdown sales report from raw row dicts.

    This orchestrator is intentionally slim (SRP): normalization,
    aggregation, and rendering are delegated to dedicated modules.
    """
    cleaned = normalize_rows(rows)
    totals = totals_by_category(cleaned)
    return render_markdown(totals)
