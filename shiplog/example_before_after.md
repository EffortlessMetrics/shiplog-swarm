# Random function broken down with DRY + SRP

Picked function concept at random: **shiplog entry processing**.

- Before: one all-in-one parser/validator/normalizer function.
- After:
  - `parser.py`: parsing only
  - `validator.py`: validation only
  - `normalizer.py`: normalization only
  - `processor.py`: orchestration only

This keeps responsibilities focused and reusable.
