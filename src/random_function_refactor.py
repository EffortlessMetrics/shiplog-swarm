"""Example refactor: break one function into DRY, SRP-oriented submodules.

This file started from a single function that mixed parsing, validation,
calculation, and formatting for order summaries. It is now decomposed into
small focused helpers.
"""

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class OrderLine:
    name: str
    quantity: int
    unit_price: float


TAX_RATE = 0.0825
DISCOUNT_THRESHOLD = 100.0
DISCOUNT_RATE = 0.10


def parse_order_line(raw_line: str) -> OrderLine:
    """Convert `name,quantity,unit_price` into an OrderLine."""
    name, quantity, unit_price = [part.strip() for part in raw_line.split(",")]
    return OrderLine(name=name, quantity=int(quantity), unit_price=float(unit_price))


def validate_line(line: OrderLine) -> None:
    """Validate a single order line."""
    if not line.name:
        raise ValueError("Item name cannot be empty.")
    if line.quantity <= 0:
        raise ValueError(f"Quantity for {line.name!r} must be positive.")
    if line.unit_price < 0:
        raise ValueError(f"Unit price for {line.name!r} cannot be negative.")


def line_total(line: OrderLine) -> float:
    """Calculate pre-tax total for one line."""
    return line.quantity * line.unit_price


def subtotal(lines: Iterable[OrderLine]) -> float:
    """Aggregate line totals."""
    return sum(line_total(line) for line in lines)


def discount_for(subtotal_amount: float) -> float:
    """Compute discount based on subtotal policy."""
    return subtotal_amount * DISCOUNT_RATE if subtotal_amount >= DISCOUNT_THRESHOLD else 0.0


def tax_for(taxable_amount: float) -> float:
    """Compute tax amount."""
    return taxable_amount * TAX_RATE


def format_currency(value: float) -> str:
    """Centralized currency formatter to stay DRY."""
    return f"${value:.2f}"


def summarize_order(raw_lines: Iterable[str]) -> str:
    """Create a human-readable order summary from raw line input."""
    lines = [parse_order_line(raw) for raw in raw_lines]
    for line in lines:
        validate_line(line)

    order_subtotal = subtotal(lines)
    discount = discount_for(order_subtotal)
    taxable = order_subtotal - discount
    tax = tax_for(taxable)
    total = taxable + tax

    return (
        f"Subtotal: {format_currency(order_subtotal)}\n"
        f"Discount: {format_currency(discount)}\n"
        f"Tax: {format_currency(tax)}\n"
        f"Total: {format_currency(total)}"
    )
