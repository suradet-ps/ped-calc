# Contributing to PedCalc

First off, thank you for considering contributing to PedCalc! It's people like you that make PedCalc such a great tool.

The following is a set of guidelines for contributing to PedCalc. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## How Can I Contribute?

### Reporting Bugs

This section guides you through submitting a bug report for PedCalc. Following these guidelines helps maintainers and the community understand your report, reproduce the behavior, and find related reports.

- **Check if the bug has already been reported:** Before creating a new issue, check the issues list to see if the problem has already been reported. If it has, add a comment to the existing issue instead of opening a new one.
- **Provide a clear and descriptive title:** Use a clear and descriptive title for the issue to identify the problem.
- **Describe the exact steps to reproduce the problem:** Provide the exact steps to reproduce the problem in as much detail as possible.
- **Provide specific examples:** Provide specific examples to demonstrate the steps.
- **Describe the behavior you observed:** Describe the behavior you observed after following the steps and point out what exactly is the problem with that behavior.
- **Explain which behavior you expected to see instead and why:** Explain which behavior you expected to see instead and why.

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for PedCalc, including completely new features and minor improvements to existing functionality. Following these guidelines helps maintainers and the community understand your suggestion and find related suggestions.

- **Check if the enhancement has already been suggested:** Before creating a new issue, check the issues list to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- **Provide a clear and descriptive title:** Use a clear and descriptive title for the issue to identify the suggestion.
- **Provide a step-by-step description of the suggested enhancement:** Provide a step-by-step description of the suggested enhancement in as much detail as possible.
- **Provide specific examples:** Provide specific examples to demonstrate the steps.
- **Describe the current behavior and explain which behavior you expected to see instead and why:** Describe the current behavior and explain which behavior you expected to see instead and why.
- **Explain why this enhancement would be useful:** Explain why this enhancement would be useful to most PedCalc users.

### Pull Requests

The process described here has several goals:

- Maintain PedCalc's quality
- Fix problems that are important to users
- Engage the community in working toward the best possible PedCalc
- Enable a sustainable system for PedCalc's maintainers to review contributions

Please follow these steps to have your contribution considered by the maintainers:

1. Follow all instructions in the template
2. Follow the styleguides
3. After you submit your pull request, verify that all status checks are passing

While the prerequisites above must be satisfied prior to having your pull request reviewed, the reviewer(s) may ask you to complete additional design work, tests, or other changes before your pull request can be ultimately accepted.

## Environment Setup

To contribute to PedCalc, you will need the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2021)
- [Trunk](https://trunkrs.dev/) (WASM application bundler)
- The WebAssembly compile target (`wasm32-unknown-unknown`)

### Setup Steps

1. Fork the repository
2. Clone your fork: `git clone https://github.com/suradet-ps/ped-calc.git`
3. Add the WebAssembly target: `rustup target add wasm32-unknown-unknown`
4. Install Trunk: `cargo install trunk`
5. Start the development server: `trunk serve`

## Styleguides

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

### Rust Styleguide

All Rust code should be formatted using `cargo fmt`. Please ensure your code follows standard Rust conventions.

---

Thank you for your interest in contributing to PedCalc
