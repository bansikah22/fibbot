use regex::Regex;

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
