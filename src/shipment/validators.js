function assertRequiredInputs(order, pricingRules) {
  if (!order || !pricingRules) {
    throw new Error('order and pricingRules are required');
  }
}

module.exports = { assertRequiredInputs };
