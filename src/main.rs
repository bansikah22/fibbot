use fibbot::fib::{extract_numbers, compute_fibonacci, process_pr_description};
use std::env;

fn main() {
    // Retrieve input parameters from GitHub Actions environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());

    // Convert max_threshold to an integer
    let max_threshold: u32 = max_threshold.parse().unwrap_or(100);

    // Print parsed values (for testing)
    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Call the imported functions here
    let fib_sequence = compute_fibonacci(max_threshold);
    println!("Fibonacci Sequence: {:?}", fib_sequence);

    let pr_description = "Fixes issues 12, 34, and 56.";
    let numbers = extract_numbers(pr_description);
    println!("Extracted Numbers: {:?}", numbers);


    let pr_description = "Updated files for issues 3, 5, 8, and 13.";
    let matching_fib_numbers = process_pr_description(pr_description, max_threshold);
    println!("Matching Fibonacci Numbers: {:?}", matching_fib_numbers);
}
