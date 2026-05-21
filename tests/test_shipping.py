from src.shipping import ShipmentItem, build_invoice


def test_build_invoice_with_coupon_and_shipping():
    items = [
        ShipmentItem("label", 20, 2, taxable=True),
        ShipmentItem("mailers", 10, 1, taxable=False),
    ]

    invoice = build_invoice(items, tax_rate=0.08, coupon_code="save10")

    assert invoice.subtotal == 50.00
    assert invoice.tax == 3.20
    assert invoice.shipping == 8.99
    assert invoice.discount == 5.00
    assert invoice.total == 57.19
