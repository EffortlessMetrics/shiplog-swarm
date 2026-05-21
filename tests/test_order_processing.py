from src.order_processing import OrderInput, process_order


def test_process_order_with_coupon_and_shipping():
    result = process_order(OrderInput(price=10.0, quantity=3, coupon=0.10))

    assert result.subtotal == 30.0
    assert result.discount == 3.0
    assert result.taxed_subtotal == 29.16
    assert result.shipping == 5.0
    assert result.total == 34.16


def test_process_order_with_free_shipping():
    result = process_order(OrderInput(price=30.0, quantity=2, coupon=0.0))

    assert result.subtotal == 60.0
    assert result.shipping == 0.0
    assert result.total == 64.8
