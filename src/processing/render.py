def render_markdown(totals: dict[str, float]) -> str:
    """Render category totals to a markdown table."""
    header = "| Category | Total |\n|---|---:|"
    lines = [header]
    for category in sorted(totals):
        lines.append(f"| {category} | {totals[category]:.2f} |")
    if len(lines) == 1:
        lines.append("| _none_ | 0.00 |")
    return "\n".join(lines)
