use reqwest::Client;
use serde::Serialize;
use regex::Regex;
use std::env;

#[derive(Serialize)]
struct Comment {
    body: String,
}

/// Extracts numbers from a given PR description string.
pub fn extract_numbers(pr_description: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(pr_description)
        .filter_map(|digits| digits.as_str().parse::<u32>().ok())
        .collect()
}

/// Computes Fibonacci numbers up to a given threshold.
pub fn compute_fibonacci(n: u32) -> Vec<u32> {
    let mut fib = vec![0, 1];
    while let Some(&last) = fib.last() {
        let second_last = fib.get(fib.len() - 2).unwrap_or(&0);
        let next = last + second_last;
        if next > n {
            break;
        }
        fib.push(next);
    }
    fib
}

/// Processes a PR description, extracts numbers, and finds Fibonacci numbers within the range.
pub fn process_pr_description(pr_description: &str, max_threshold: u32) -> Vec<u32> {
    let numbers = extract_numbers(pr_description);
    let fib_sequence = compute_fibonacci(max_threshold);

    numbers
        .into_iter()
        .filter(|num| fib_sequence.contains(num))
        .collect()
}

/// Post a comment on a GitHub PR.
pub async fn post_comment(pr_number: u64, comment: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = env::var("GITHUB_REPOSITORY")?; // Ensure the GitHub repository is correctly set
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo,
        pr_number
    );

    let client = Client::new();
    let comment = Comment { body: comment.to_string() };

    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&comment)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully posted comment.");
    } else {
        println!("Failed to post comment: {}", response.status());
    }

    Ok(())
}
