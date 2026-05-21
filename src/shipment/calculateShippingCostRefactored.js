const { assertRequiredInputs } = require('./validators');
const { calculateTotalWeight } = require('./weight');
const { calculateBaseCost, getZoneMultiplier } = require('./basePricing');
const { calculateDiscount } = require('./discounts');
const { calculateFuelSurcharge } = require('./surcharges');
const { applyTax } = require('./taxation');

/**
 * SRP/DRY-oriented rewrite of calculateShippingCostLegacy.
 * Each rule is owned by a focused module.
 */
function calculateShippingCost(order, pricingRules) {
  assertRequiredInputs(order, pricingRules);

  const totalWeight = calculateTotalWeight(order.items);
  const baseCost = calculateBaseCost(totalWeight, pricingRules);
  const discount = calculateDiscount(baseCost, order);
  const fuelSurcharge = calculateFuelSurcharge(baseCost, pricingRules.fuelIndex);
  const preTaxTotal = baseCost - discount + fuelSurcharge;
  const taxedTotal = applyTax(preTaxTotal, pricingRules.taxRate);
  const zoneMultiplier = getZoneMultiplier(
    order.destinationZone,
    pricingRules.zoneMultipliers
  );

  return Math.max(0, taxedTotal * zoneMultiplier);
}

module.exports = { calculateShippingCost };
