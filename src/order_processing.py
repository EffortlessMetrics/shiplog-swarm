"""Order processing helpers split into SRP-focused functions."""

from __future__ import annotations

from dataclasses import dataclass

TAX_RATE = 0.08
FREE_SHIPPING_THRESHOLD = 50.0
STANDARD_SHIPPING_FEE = 5.0


@dataclass(frozen=True)
class OrderInput:
    price: float
    quantity: int
    coupon: float = 0.0


@dataclass(frozen=True)
class OrderBreakdown:
    subtotal: float
    discount: float
    taxed_subtotal: float
    shipping: float
    total: float


def process_order(order: OrderInput) -> OrderBreakdown:
    """Orchestrate order pricing steps.

    Kept intentionally tiny so each pricing concern lives in one place.
    """
    subtotal = calculate_subtotal(order.price, order.quantity)
    discount = calculate_discount(subtotal, order.coupon)
    taxed_subtotal = apply_tax(subtotal - discount)
    shipping = calculate_shipping(taxed_subtotal)
    total = round_currency(taxed_subtotal + shipping)
    return OrderBreakdown(subtotal, discount, taxed_subtotal, shipping, total)


def calculate_subtotal(price: float, quantity: int) -> float:
    return round_currency(price * quantity)


def calculate_discount(subtotal: float, coupon: float) -> float:
    if coupon <= 0:
        return 0.0
    capped_coupon = min(coupon, 0.30)
    return round_currency(subtotal * capped_coupon)


def apply_tax(amount: float) -> float:
    return round_currency(amount * (1 + TAX_RATE))


def calculate_shipping(amount_after_tax: float) -> float:
    if amount_after_tax >= FREE_SHIPPING_THRESHOLD:
        return 0.0
    return STANDARD_SHIPPING_FEE


def round_currency(value: float) -> float:
    return round(value, 2)
