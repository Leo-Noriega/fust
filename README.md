# Fust

[![CI](https://github.com/noriega/fust/workflows/CI/badge.svg)](https://github.com/noriega/fust/actions)
[![codecov](https://codecov.io/gh/noriega/fust/branch/main/graph/badge.svg)](https://codecov.io/gh/noriega/fust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://forge.rust-lang.org/)

> A modern CLI application built with Rust following best practices

## Features

- 🚀 High-performance Rust implementation
- 🔧 Clean architecture with hexagonal/port-adapter pattern
- 🧪 Test-driven development (TDD)
- 📝 Comprehensive documentation
- 🔍 Static analysis with Clippy
- 🎨 Consistent formatting with rustfmt
- 🛡️ Security auditing
- 📊 Code coverage tracking
- ⚡ Optimized CI/CD for GitHub Free tier

## Development

### Prerequisites

- Rust 1.70.0 or later
- Cargo

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## CI/CD Pipeline

Our CI is optimized for GitHub Free tier (2,000 minutes/month):

- **Code Quality**: Check, fmt, clippy, security audit (combined job)
- **Testing**: Multi-platform tests (Ubuntu, macOS)
- **Coverage**: Code coverage on PRs only
- **Release**: Build verification on main branch

**Estimated usage**: ~15-20 minutes per push/PR

## Architecture

This project follows hexagonal architecture principles with:

- **Domain Layer**: Core business logic
- **Application Layer**: Use cases and orchestration  
- **Infrastructure Layer**: External adapters (CLI, file system, etc.)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Ensure CI passes
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 