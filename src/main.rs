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
}
