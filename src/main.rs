use fibbot::fib::{compute_fibonacci, process_pr_description, post_comment, format_comment, extract_numbers};
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
    
    // Get the PR description, title and content
    let (pr_description, pr_title) = match env::var("GITHUB_EVENT_PATH") {
        Ok(event_path) => {
            match std::fs::read_to_string(event_path) {
                Ok(contents) => {
                    match serde_json::from_str::<serde_json::Value>(&contents) {
                        Ok(event) => {
                            let body = event["pull_request"]["body"].as_str().unwrap_or("").to_string();
                            let title = event["pull_request"]["title"].as_str().unwrap_or("").to_string();
                            (body, title)
                        },
                        Err(_) => ("".to_string(), "".to_string())
                    }
                },
                Err(_) => ("".to_string(), "".to_string())
            }
        },
        Err(_) => ("".to_string(), "".to_string())
    };
    
    // Combine title and description for number extraction
    let pr_content = format!("{} {}", pr_title, pr_description);
    
    // Print parsed values (for testing)
    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
    println!("PR Number: {}", pr_number);
    println!("PR Title: {}", pr_title);
    println!("PR Description: {}", pr_description);
    println!("GitHub Token length: {}", github_token.len());
    
    // Compute Fibonacci Sequence
    if enable_fib == "true" {
        let fib_sequence = compute_fibonacci(max_threshold);
        println!("Fibonacci Sequence up to {}: {:?}", max_threshold, fib_sequence);
        
        // Extract all numbers from PR content
        let all_numbers = extract_numbers(&pr_content);
        println!("All numbers found in PR: {:?}", all_numbers);
        
        // Process PR content to get Fibonacci and non-Fibonacci numbers
        let (fibonacci_numbers, non_fibonacci_numbers) = process_pr_description(&pr_content, max_threshold);
        
        println!("Fibonacci Numbers: {:?}", fibonacci_numbers);
        println!("Non-Fibonacci Numbers: {:?}", non_fibonacci_numbers);
        
        // Generate a formatted comment
        let comment = format_comment(
            &fibonacci_numbers,
            &non_fibonacci_numbers,
            &all_numbers,
            max_threshold
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
            // Print the comment to console for debugging
            println!("Comment that would be posted:\n{}", comment);
        }
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}