use fibbot::fib::{compute_fibonacci, process_pr_description, post_comment};
use std::env;
use tokio;
use serde_json;

#[tokio::main]
async fn main() {
    // Retrieve input parameters from GitHub Actions environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
    
    // Convert max_threshold to an integer
    let max_threshold: u32 = max_threshold.parse().unwrap_or(100);
    
    // Ensure GitHub token is available
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());
    
    // Print environment variables for debugging
    println!("Environment variables:");
    for (key, value) in env::vars() {
        if key.contains("GITHUB") {
            println!("  {} = {}", key, value);
        }
    }
    
    // Get the PR number from the GitHub event
    let pr_number = match env::var("GITHUB_EVENT_PATH") {
        Ok(event_path) => {
            println!("Reading event from: {}", event_path);
            match std::fs::read_to_string(event_path) {
                Ok(contents) => {
                    println!("Event content length: {} bytes", contents.len());
                    match serde_json::from_str::<serde_json::Value>(&contents) {
                        Ok(event) => {
                            let number = event["pull_request"]["number"].as_u64().unwrap_or(0);
                            println!("Parsed PR number: {}", number);
                            number
                        },
                        Err(e) => {
                            println!("Error parsing event JSON: {}", e);
                            0
                        }
                    }
                },
                Err(e) => {
                    println!("Error reading event file: {}", e);
                    0
                }
            }
        },
        Err(e) => {
            println!("Error getting GITHUB_EVENT_PATH: {}", e);
            0
        }
    };
    
    // Get the PR description
    let pr_description = match env::var("GITHUB_EVENT_PATH") {
        Ok(event_path) => {
            match std::fs::read_to_string(event_path) {
                Ok(contents) => {
                    match serde_json::from_str::<serde_json::Value>(&contents) {
                        Ok(event) => {
                            event["pull_request"]["body"].as_str().unwrap_or("").to_string()
                        },
                        Err(_) => "".to_string()
                    }
                },
                Err(_) => "".to_string()
            }
        },
        Err(_) => "".to_string()
    };
    
    // Print parsed values (for testing)
    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
    println!("PR Number: {}", pr_number);
    println!("PR Description: {}", pr_description);
    println!("GitHub Token length: {}", github_token.len());
    
    // Compute Fibonacci Sequence
    if enable_fib == "true" {
        let fib_sequence = compute_fibonacci(max_threshold);
        println!("Fibonacci Sequence: {:?}", fib_sequence);
        
        // Extract PR numbers and match against Fibonacci sequence
        let matching_fib_numbers = process_pr_description(&pr_description, max_threshold);
        println!("Matching Fibonacci Numbers: {:?}", matching_fib_numbers);
        
        // Post a comment on GitHub with the matching Fibonacci numbers
        let comment = format!(
            "FibBot Analysis Result:\n\nThe following numbers from the PR description are Fibonacci numbers: {:?}",
            matching_fib_numbers
        );
        
        if !github_token.is_empty() && pr_number > 0 {
            // Only attempt to post a comment if the token is available and PR number is valid
            match post_comment(pr_number, &comment, &github_token).await {
                Ok(_) => println!("Successfully posted the comment."),
                Err(err) => eprintln!("Failed to post comment: {}", err),
            }
        } else {
            if github_token.is_empty() {
                println!("GitHub token is missing.");
            }
            if pr_number == 0 {
                println!("Invalid PR number.");
            }
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}