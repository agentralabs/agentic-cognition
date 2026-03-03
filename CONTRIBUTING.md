# Contributing to AgenticCognition

Thank you for your interest in contributing to AgenticCognition!

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).

## How to Contribute

### Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Include reproduction steps for bugs
- Include version information (`acog version`)

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Follow existing code patterns and naming conventions
4. Write tests for new functionality
5. Ensure `cargo test --all-features` passes
6. Ensure `cargo clippy --all-targets -- -D warnings` passes
7. Use conventional commit messages (`feat:`, `fix:`, `chore:`, `docs:`)
8. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- No `.unwrap()` in library code
- Strict validation (no silent fallbacks)
- All public items must have doc comments

### Testing

- Unit tests in `tests/unit/`
- Integration tests in `tests/integration/`
- MCP tests in `tests/mcp/`
- Run all: `cargo test --all-features`

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
