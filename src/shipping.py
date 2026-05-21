from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class ShipmentItem:
    name: str
    unit_price: float
    quantity: int
    taxable: bool = True


@dataclass(frozen=True)
class Invoice:
    subtotal: float
    tax: float
    shipping: float
    discount: float
    total: float


def _calculate_subtotal(items: Iterable[ShipmentItem]) -> float:
    return round(sum(item.unit_price * item.quantity for item in items), 2)


def _calculate_tax(items: Iterable[ShipmentItem], tax_rate: float) -> float:
    taxable_subtotal = sum(
        item.unit_price * item.quantity for item in items if item.taxable
    )
    return round(taxable_subtotal * tax_rate, 2)


def _calculate_shipping(subtotal: float) -> float:
    return 0.0 if subtotal >= 100 else 8.99


def _calculate_discount(subtotal: float, coupon_code: str | None) -> float:
    if not coupon_code:
        return 0.0
    coupons = {
        "SAVE10": 0.10,
        "SAVE20": 0.20,
    }
    rate = coupons.get(coupon_code.upper(), 0.0)
    return round(subtotal * rate, 2)


def build_invoice(
    items: Iterable[ShipmentItem],
    tax_rate: float,
    coupon_code: str | None = None,
) -> Invoice:
    """SRP-oriented replacement for monolithic invoice calculation."""
    items = list(items)
    subtotal = _calculate_subtotal(items)
    tax = _calculate_tax(items, tax_rate)
    shipping = _calculate_shipping(subtotal)
    discount = _calculate_discount(subtotal, coupon_code)
    total = round(subtotal + tax + shipping - discount, 2)
    return Invoice(
        subtotal=subtotal,
        tax=tax,
        shipping=shipping,
        discount=discount,
        total=total,
    )
