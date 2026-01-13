# Contributing to clock-rand

Thank you for your interest in contributing to clock-rand! We welcome contributions from the community.

## Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/Olyntar-Labs/clock-rand.git
   cd clock-rand
   ```

2. **Install Rust**
   - Install the latest stable Rust toolchain from [rustup.rs](https://rustup.rs/)

3. **Run tests**
   ```bash
   cargo test --all-features
   ```

4. **Check code quality**
   ```bash
   cargo clippy --all-features -- -D warnings
   cargo fmt -- --check
   ```

## Code Guidelines

- Follow the existing code style (enforced by `cargo fmt`)
- Add documentation for all public APIs
- Write tests for new functionality
- Update documentation for any breaking changes
- Ensure all tests pass before submitting

## Commit Messages

Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `test:` for test additions/changes
- `refactor:` for code refactoring
- `chore:` for maintenance tasks

Example: `feat: add ChaCha20Rng implementation`

## Pull Request Process

1. Ensure your code follows the guidelines above
2. Update the README.md if needed
3. Add appropriate tests
4. Ensure CI passes
5. Request review from maintainers

## Security Considerations

- Cryptographic code must be thoroughly reviewed
- Any changes to RNG implementations need extensive testing
- Report security issues privately to [security@olyntar.com](mailto:security@olyntar.com)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).