# bestls Pull Request Review Checklist ✅

This checklist is for reviewers to ensure all pull requests follow the project's standards and maintain code quality.

---

## 🧩 Functionality

- [ ] Code works as intended
- [ ] Handles edge cases correctly
- [ ] Proper error handling for unexpected input
- [ ] No regressions introduced

---

## 🧹 Code Quality

- [ ] Code is readable and well-structured
- [ ] Functions are appropriately sized
- [ ] Variables and functions have meaningful names
- [ ] Comments explain complex or non-obvious logic
- [ ] No unnecessary or redundant code
- [ ] Follows project style guidelines (formatting, indentation, naming)

---

## 🧪 Testing

- [ ] Adequate unit/integration test coverage for new/modified code
- [ ] Manual testing performed where automated tests are insufficient
- [ ] Tests pass locally and in CI
- [ ] No new warnings or errors introduced

---

## 📚 Documentation

- [ ] Public APIs and modules are documented
- [ ] README updated if functionality changes
- [ ] Inline code comments are helpful and clear
- [ ] Breaking changes are documented

---

## ⚙️ CI/CD & Workflow

- [ ] Pull request is up-to-date with target branch
- [ ] CI checks (multi-platform, linting, formatting, security audit) pass
- [ ] Pull request type label (`feat`, `fix`, `docs`, etc.) is correct
- [ ] PR title format follows: `<type>(<scope>): <description>`

---

## 📝 Optional

- [ ] Screenshots or GIFs added if UI/UX changes
- [ ] Performance implications documented (if applicable)
- [ ] Additional notes for future maintainers

---

**Usage:**
Check each item during code review. If an item is not applicable, mark it as N/A in comments. All critical items should be addressed before approval.
