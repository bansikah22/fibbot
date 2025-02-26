use fibbot::fib::{extract_numbers, compute_fibonacci, process_pr_description};

#[test]
fn test_extract_numbers() {
    let input = "Fixes issues 12, 34, and 56.";
    assert_eq!(extract_numbers(input), vec![12, 34, 56]);
}

#[test]
fn test_extract_numbers_empty() {
    let input = "No numbers here.";
    assert_eq!(extract_numbers(input), Vec::<u32>::new());
}

#[test]
fn test_fibonacci_sequence() {
    assert_eq!(compute_fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8]);
}

#[test]
fn test_process_pr_description() {
    let pr_description = "Updated files for issues 3, 5, 8, and 13.";
    assert_eq!(process_pr_description(pr_description, 20), vec![3, 5, 8, 13]);
}
