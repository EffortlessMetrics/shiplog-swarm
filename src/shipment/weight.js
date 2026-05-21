function calculateTotalWeight(items = []) {
  return items.reduce((sum, item) => sum + item.weight * item.quantity, 0);
}

module.exports = { calculateTotalWeight };
