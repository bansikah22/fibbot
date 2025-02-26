use fibbot::fib::{compute_fibonacci, process_pr_description, post_comment};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    // Retrieve input parameters from GitHub Actions environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());

    // Convert max_threshold to an integer
    let max_threshold: u32 = max_threshold.parse().unwrap_or(100);

    // Ensure GitHub token is available
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());

    // Get the PR number from the GitHub context (GITHUB_REF) to determine which PR we are commenting on
    let pr_number = env::var("GITHUB_REF")
        .unwrap_or_else(|_| "refs/pull/0/merge".to_string())
        .split('/')
        .nth(2)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);

    // Print parsed values (for testing)
    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Compute Fibonacci Sequence
    if enable_fib == "true" {
        let fib_sequence = compute_fibonacci(max_threshold);
        println!("Fibonacci Sequence: {:?}", fib_sequence);

        // Extract PR numbers and match against Fibonacci sequence
        let pr_description = "Fixes issues 12, 34, and 56."; // Replace with the PR description
        let matching_fib_numbers = process_pr_description(pr_description, max_threshold);
        println!("Matching Fibonacci Numbers: {:?}", matching_fib_numbers);

        // Post a comment on GitHub with the matching Fibonacci numbers
        let comment = format!(
            "The following numbers from the PR description are Fibonacci numbers: {:?}",
            matching_fib_numbers
        );
        if !github_token.is_empty() {
            // Only attempt to post a comment if the token is available
            match post_comment(pr_number, &comment, &github_token).await {
                Ok(_) => println!("Successfully posted the comment."),
                Err(err) => eprintln!("Failed to post comment: {}", err),
            }
        } else {
            println!("GitHub token is missing.");
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}
