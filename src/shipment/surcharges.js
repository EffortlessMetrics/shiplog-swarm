function calculateFuelSurcharge(baseCost, fuelIndex) {
  if (fuelIndex > 1.2) {
    return baseCost * 0.08;
  }

  return 0;
}

module.exports = { calculateFuelSurcharge };
