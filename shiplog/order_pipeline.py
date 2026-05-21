"""Order processing pipeline organized by SRP-focused helpers."""

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class LineItem:
    sku: str
    quantity: int
    unit_price: float


@dataclass(frozen=True)
class Order:
    order_id: str
    customer_id: str
    line_items: tuple[LineItem, ...]
    coupon_code: str | None = None


TAX_RATE = 0.0825
COUPON_DISCOUNTS = {
    "SHIP10": 0.10,
    "SHIP20": 0.20,
}


def process_order(order: Order) -> dict[str, float | str]:
    """Return a final bill breakdown for a customer order."""
    subtotal = calculate_subtotal(order.line_items)
    discount = calculate_discount(subtotal, order.coupon_code)
    taxed_amount = apply_tax(subtotal - discount)
    total = round_currency(taxed_amount)

    return {
        "order_id": order.order_id,
        "customer_id": order.customer_id,
        "subtotal": round_currency(subtotal),
        "discount": round_currency(discount),
        "tax": round_currency(taxed_amount - (subtotal - discount)),
        "total": total,
    }


def calculate_subtotal(items: Iterable[LineItem]) -> float:
    return sum(item.quantity * item.unit_price for item in items)


def calculate_discount(subtotal: float, coupon_code: str | None) -> float:
    if not coupon_code:
        return 0.0

    return subtotal * COUPON_DISCOUNTS.get(coupon_code, 0.0)


def apply_tax(amount: float) -> float:
    return amount * (1 + TAX_RATE)


def round_currency(amount: float) -> float:
    return round(amount, 2)
