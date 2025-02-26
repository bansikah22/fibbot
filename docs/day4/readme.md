# ✅ Day 4: Core Logic Development - Completion Checklist

## **Milestone 1: Extract Numbers from PR Content**
- [x] Implemented a function to parse PR descriptions and extract numbers.
  - Used Rust’s `regex` crate to identify and extract numbers from PR descriptions.
  - Ensured that only valid numbers are extracted, and errors are handled gracefully.

## **Milestone 2: Implement Fibonacci Calculator**
- [x] Developed a Rust function to compute Fibonacci numbers.
  - Used an efficient algorithm to handle both small and large Fibonacci numbers.
  - Added edge case handling for inputs like 0 and 1, ensuring proper results.

- [x] Validated the Fibonacci function with sample test cases:
  - Confirmed that the function correctly computes Fibonacci sequences for a wide range of inputs.

## **Milestone 3: Local Testing of Core Logic**
- [x] Tested the number extraction and Fibonacci calculations locally:
  - Extracted numbers from mock PR content.
  - Ran the Fibonacci function with different thresholds, confirming correct output for both small and large numbers.

- [x] **To test locally**, used mock PR descriptions to simulate the action:
  - PR description: "Calculate Fibonacci for 10 and 50."
  - Expected output: Fibonacci numbers for `10` and `50` correctly calculated.

## **Verification**
- [x] Ensured that the number extraction correctly identifies and parses valid numbers from PR descriptions.
- [x] Validated the Fibonacci calculation logic with various inputs, including edge cases.
- [x] Confirmed that both the number extraction and Fibonacci computation work as expected in a local test environment.
- [x] Verified that the logic is ready for integration with GitHub Actions.

✅ **Day 4 successfully completed!** The core logic for number extraction and Fibonacci computation is fully implemented and working. 🚀
