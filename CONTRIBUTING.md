# Contributing to bestls

Thank you for your interest in contributing to bestls! We welcome contributions from everyone. This document provides guidelines and steps for contributing.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to [mkm9284@gmail.com](mailto:mkm9284@gmail.com).

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues list as you might find that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- Use a clear and descriptive title
- Describe the exact steps which reproduce the problem
- Provide specific examples to demonstrate the steps
- Describe the behavior you observed after following the steps
- Explain which behavior you expected to see instead and why
- Include your OS and bestls version (`bestls --version`)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- A clear and descriptive title
- A detailed description of the proposed functionality
- Explain why this enhancement would be useful
- List some other tools where this enhancement exists, if applicable

### Pull Requests

- Fill in the required template
- Follow our coding style
- Include appropriate tests
- Update documentation for significant changes
- End all files with a newline

## Development Process

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run clippy (`cargo clippy`)
6. Format your code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add some amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## Styleguides

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

### Rust Styleguide

- Follow the official Rust style guide
- Use `cargo fmt` before committing
- Ensure `cargo clippy` shows no warnings
- Document public API elements using rustdoc
- Write clear, descriptive variable and function names
- Keep functions focused and reasonably sized

### Documentation Styleguide

- Use Markdown
- Document all public APIs
- Include examples in documentation when applicable
- Keep line length to a maximum of 80 characters
- Use descriptive link texts, avoid "click here"

## Additional Notes

### Issue and Pull Request Labels

- `bug` - confirmed bugs or reports that are very likely to be bugs
- `enhancement` - new feature or request
- `documentation` - documentation improvements
- `good first issue` - good for newcomers
- `help wanted` - extra attention is needed
- `wontfix` - will not be worked on
