# ✅ GitHub Action in Rust - 6-Day Development Checklist

## **Day 1: Kickoff & Action Review**

- [ ] **Review GitHub Actions Fundamentals**
  - [ ] Summary or checklist of key concepts on creating custom GitHub Actions (especially for non-JavaScript implementations).
- [ ] **Project Initialization**
  - [ ] Create a new Rust project (`cargo new fibbot`).
  - [ ] Ensure the repository structure is set up correctly.
  - [ ] Add an `action.yml` file with basic metadata.
- [ ] **Verification**
  - [ ] Ensure all files are committed and pushed to GitHub.
  - [ ] Confirm understanding of how GitHub Actions and `action.yml` work.

---

## **Day 2: Minimal Action Implementation**

- [ ] **Build a “Hello World” Action in Rust**
  - [ ] Create a Rust program that prints "Hello, world!".
- [ ] **Test the Basic Action**
  - [ ] Add a GitHub workflow to trigger the action.
  - [ ] Ensure "Hello, world!" output appears in the action logs.
- [ ] **Verification**
  - [ ] Check the GitHub Actions logs for successful execution.

---

## **Day 3: Parameter Handling & Input Parsing**

- [ ] **Define and Parse Inputs**
  - [ ] Update `action.yml` to include `enable_fib` and `max_threshold` parameters.
  - [ ] Implement input parsing in Rust.
- [ ] **Local Testing of Parameters**
  - [ ] Validate that parameters are correctly read from the workflow.
  - [ ] Print/log values to confirm correctness.
- [ ] **Verification**
  - [ ] Run a test workflow with different input values and confirm they are received correctly.

---

## **Day 4: Core Logic Development**

- [ ] **Extract Numbers from PR Content**
  - [ ] Implement a function to parse PR descriptions and extract numbers.
- [ ] **Implement Fibonacci Calculator**
  - [ ] Develop a Rust function to compute Fibonacci numbers.
  - [ ] Ensure it handles edge cases and large numbers efficiently.
- [ ] **Verification**
  - [ ] Write unit tests for number extraction and Fibonacci calculations.
  - [ ] Confirm correct functionality via test cases.

---

## **Day 5: Integration & GitHub API Interaction**

- [ ] **Combine Parsing & Calculation**
  - [ ] Integrate number extraction and Fibonacci computation.
- [ ] **Interact with GitHub API**
  - [ ] Implement GitHub API authentication.
  - [ ] Develop functionality to post a comment on a pull request.
- [ ] **Verification**
  - [ ] Test with a real pull request and verify the action posts a comment.

---

## **Day 6: Full Workflow Testing & Documentation**

- [ ] **End-to-End Workflow Testing**
  - [ ] Ensure the entire action works on a test repository.
  - [ ] Debug any final issues.
- [ ] **Documentation & Final Touches**
  - [ ] Write a README with setup instructions and parameter configurations.
  - [ ] Add comments and improve code quality.
  - [ ] Ensure the action is reusable and configurable.
- [ ] **Verification**
  - [ ] Final GitHub Action successfully runs in production.

---

🎯 **By the end of Day 6, you should have a fully functional, documented GitHub Action that scans PRs for numbers, calculates Fibonacci sequences, and posts results as comments.**
