# Contributing to bestls

Thank you for your interest in contributing to **bestls**! We welcome contributions from everyone. This document provides guidelines and steps for contributing, along with coding standards, workflow expectations, and review practices.

---

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior to [mkm9284@gmail.com](mailto:mkm9284@gmail.com).

---

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, check the existing issues list—you might find your issue has already been reported. When creating a bug report, please include:

- Clear and descriptive title
- Exact steps to reproduce the issue
- Specific examples demonstrating the steps
- Observed behavior after following the steps
- Expected behavior and why
- OS and `bestls` version (`bestls --version`)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. Include the following:

- Clear and descriptive title
- Detailed description of the proposed functionality
- Why this enhancement would be useful
- Reference to other tools where this exists (if applicable)

### Pull Requests

When submitting a pull request (PR), please follow these guidelines to ensure smooth review and integration:

#### **PR Title Format:**

Use the following format for PR titles to keep a consistent history:

```
<type>(<scope>): <description>

Examples:
feat(tree): add recursive directory listing
fix(table): correct permission display formatting
docs(readme): update installation instructions
ci(workflow): add security audit step
```

#### **PR Guidelines:**

- Fill in the required PR template completely
- Follow the project coding style guidelines
- Include appropriate tests for new features or bug fixes
- Update documentation for any significant changes
- Ensure all files end with a newline
- Keep your branch up-to-date with the target branch before merging
- Assign relevant labels to your PR:
  - `feat` - New features
  - `fix` - Bug fixes
  - `docs` - Documentation changes
  - `style` - Code style changes (formatting, etc.)
  - `refactor` - Code refactoring
  - `perf` - Performance improvements
  - `test` - Test additions or modifications
  - `ci` - CI/CD changes
  - `chore` - Maintenance tasks

> **Reviewer Note:** Please follow the [Pull Request Review Checklist](docs/REVIEW_CHECKLIST.md) when reviewing pull requests.

---

## Development Process

1. Fork the repository
2. Create your feature branch:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. Make your changes
4. Run tests:
   ```bash
   cargo test
   ```
5. Run linter:
   ```bash
   cargo clippy
   ```
6. Format your code:
   ```bash
   cargo fmt
   ```
7. Commit your changes:
   ```bash
   git commit -m 'Add some amazing feature'
   ```
8. Push to the branch:
   ```bash
   git push origin feature/amazing-feature
   ```
9. Open a Pull Request following the template and checklist

---

## Styleguides

### Git Commit Messages

- Use present tense (`Add feature` not `Added feature`)
- Use imperative mood (`Move cursor` not `Moves cursor`)
- Limit the first line to 72 characters or less
- Reference issues and PRs after the first line

### Rust Styleguide

- Follow the official Rust style guide
- Use `cargo fmt` before committing
- Ensure `cargo clippy` shows no warnings
- Document public API elements with rustdoc
- Use clear, descriptive variable and function names
- Keep functions focused and reasonably sized

### Documentation Styleguide

- Use Markdown
- Document all public APIs
- Include examples when applicable
- Keep line length ≤ 80 characters
- Use descriptive link texts

---

## Issue and Pull Request Labels

**Type Labels:**

- `bug` - confirmed or likely bugs
- `enhancement` - new features or requests
- `documentation` - improvements to docs
- `good first issue` - suitable for newcomers
- `help wanted` - extra attention needed
- `wontfix` - will not be addressed

**Priority Labels:**

- `priority: high` - critical issues
- `priority: medium` - important but not urgent
- `priority: low` - minor or nice-to-have

**Status Labels:**

- `status: reviewing` - under review
- `status: in-progress` - work in progress
- `status: blocked` - blocked by dependencies
- `status: needs-info` - needs more information

**Component Labels:**

- `component: cli` - CLI interface
- `component: table` - Table formatting
- `component: json` - JSON output
- `component: filesystem` - File operations
- `component: tests` - Testing related

**Version Labels:**

- `v1.3` - planned for version 1.3
- `v1.4` - planned for version 1.4
- `backport` - needs backporting

---

## Additional Notes

- Follow [Pull Request Review Checklist](docs/REVIEW_CHECKLIST.md) for reviewer guidance
- Always make sure your PR passes CI checks and is up-to-date with the target branch
- Be respectful, constructive, and helpful in all interactions

---

**Happy coding! 🦀✨**
