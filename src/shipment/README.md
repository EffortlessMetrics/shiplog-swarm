# Shipping cost example refactor

Randomly selected target function: `calculateShippingCostLegacy`.

## What changed

The original single-function implementation was split into SRP modules:

- `validators.js` – input contract checks.
- `weight.js` – aggregate weight calculation.
- `basePricing.js` – base cost and zone multiplier concerns.
- `discounts.js` – discount policy rules.
- `surcharges.js` – surcharge policy rules.
- `taxation.js` – tax application.
- `calculateShippingCostRefactored.js` – orchestration composition layer.

This keeps logic DRY by isolating rule math to one place per concern.
