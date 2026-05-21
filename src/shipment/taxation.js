function applyTax(amount, taxRate) {
  return amount * (1 + taxRate);
}

module.exports = { applyTax };
