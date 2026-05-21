from __future__ import annotations

from .submodules.normalizer import normalize_rows
from .submodules.summarizer import summarize_orders
from .submodules.renderer import render_report


def build_order_report(raw_rows: list[dict]) -> str:
    """High-level orchestration kept intentionally thin (SRP)."""
    normalized_rows = normalize_rows(raw_rows)
    summary = summarize_orders(normalized_rows)
    return render_report(summary)
