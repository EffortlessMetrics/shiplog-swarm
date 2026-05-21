/**
 * Monolithic implementation retained for reference.
 * Kept to show the before/after extraction.
 */
function calculateShippingCostLegacy(order, pricingRules) {
  if (!order || !pricingRules) {
    throw new Error('order and pricingRules are required');
  }

  let subtotal = 0;
  for (const item of order.items || []) {
    subtotal += item.weight * item.quantity;
  }

  const zoneMultiplier = pricingRules.zoneMultipliers[order.destinationZone] || 1;
  const baseCost = pricingRules.baseRate + subtotal * pricingRules.ratePerWeightUnit;

  let discount = 0;
  if (order.customerType === 'enterprise') {
    discount += baseCost * 0.15;
  }
  if (order.itemsCount > 20) {
    discount += baseCost * 0.05;
  }

  let fuelSurcharge = 0;
  if (pricingRules.fuelIndex > 1.2) {
    fuelSurcharge = baseCost * 0.08;
  }

  const taxed = (baseCost - discount + fuelSurcharge) * (1 + pricingRules.taxRate);
  return Math.max(0, taxed * zoneMultiplier);
}

module.exports = { calculateShippingCostLegacy };
