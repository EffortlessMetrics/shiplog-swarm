function calculateDiscount(baseCost, order) {
  let discount = 0;

  if (order.customerType === 'enterprise') {
    discount += baseCost * 0.15;
  }

  if (order.itemsCount > 20) {
    discount += baseCost * 0.05;
  }

  return discount;
}

module.exports = { calculateDiscount };
