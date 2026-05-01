# Contributing to OpenSCAD MCP Server

Thanks for your interest in contributing! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/N0t4R0b0t/openscad-mcp-server.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test: `cargo test`
6. Format: `cargo fmt`
7. Lint: `cargo clippy`
8. Commit with clear messages
9. Push to your fork
10. Open a Pull Request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install OpenSCAD
# macOS: brew install openscad
# Ubuntu/Debian: sudo apt-get install openscad
# Windows: Download from https://openscad.org/downloads.html

# Clone and build
git clone https://github.com/N0t4R0b0t/openscad-mcp-server.git
cd openscad-mcp-server
cargo build
cargo test
```

## Commit Messages

Use clear, descriptive commit messages:

- `feat: Add new tool for model analysis`
- `fix: Resolve rendering timeout issue`
- `docs: Update README with examples`
- `refactor: Simplify OpenSCAD wrapper`
- `test: Add tests for STL export`
- `chore: Update dependencies`

## Pull Request Process

1. Update README.md if needed
2. Add tests for new functionality
3. Ensure all tests pass: `cargo test`
4. Run formatter: `cargo fmt`
5. Run linter: `cargo clippy`
6. Describe changes clearly in PR description
7. Link any related issues

## Code Style

- Follow Rust conventions (use `rustfmt` and `clippy`)
- Add documentation comments for public items
- Keep functions focused and reasonably sized
- Use meaningful variable and function names

## Testing

Add tests for new features:

```bash
#[tokio::test]
async fn test_write_scad_file() {
    // Your test here
}
```

Run tests:

```bash
cargo test --lib
cargo test --doc
```

## Reporting Issues

When reporting bugs, include:

- Description of the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, OpenSCAD version)
- Relevant logs or error messages

## Feature Requests

Suggest features by opening an issue with:

- Clear description of the feature
- Why it would be useful
- Example usage if applicable
- Any potential implementation approaches

## Questions?

Feel free to open an issue or discussion. We're here to help!

## License

By contributing, you agree that your contributions will be licensed under AGPL-3.0-only.
