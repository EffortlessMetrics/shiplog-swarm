function calculateBaseCost(totalWeight, pricingRules) {
  return pricingRules.baseRate + totalWeight * pricingRules.ratePerWeightUnit;
}

function getZoneMultiplier(destinationZone, zoneMultipliers = {}) {
  return zoneMultipliers[destinationZone] || 1;
}

module.exports = { calculateBaseCost, getZoneMultiplier };
