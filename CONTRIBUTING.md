# Contributing to tropxeskoob

We welcome contributions of all kinds! Whether you're fixing a bug, suggesting a feature, or improving documentation.

## How to Contribute

1.  **Search for existing issues.** Before starting a new one, check if it's already being discussed.
2.  **Fork the repository.** Create your own copy of the project to work on.
3.  **Create a branch.** Use a descriptive name for your branch (e.g., `fix/api-pagination`).
4.  **Make your changes.** Follow the existing code style and ensure all tests pass.
5.  **Commit with conventional messages.** Use `feat:`, `fix:`, `docs:`, etc.
6.  **Push to your fork.**
7.  **Submit a pull request.** Describe your changes in detail.

## Development Environment

This project uses [devenv](https://devenv.sh/) for a consistent development experience.

```bash
# Enter the development environment
devenv shell

# Build the project
cargo build

# Run tests
cargo test
```

## Code Style

- Use `rustfmt` for formatting.
- Follow [Conventional Commits](https://www.conventionalcommits.org/).
- Ensure `cargo clippy` is clean.

## Questions?

Feel free to open an issue for any questions or discussions.
