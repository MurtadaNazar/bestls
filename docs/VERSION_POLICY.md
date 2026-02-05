# Version Management Policy

## Overview
This project uses **manual semantic versioning** with automatic CI/CD release workflows.

## Versioning Strategy

### Manual Version Control
- **Version source of truth**: `Cargo.toml` field `[package] version`
- **When to bump**: Before creating a release tag
- **Who can bump**: Project maintainers only

### Semantic Versioning (semver)
```
MAJOR.MINOR.PATCH[-prerelease][+metadata]

- MAJOR: Breaking changes to public API
- MINOR: New features (backward compatible)
- PATCH: Bug fixes and maintenance
```

### Version Examples
- `1.0.0` - Initial stable release
- `1.1.0` - New features (v1.1)
- `1.2.0` - Partial v1.2 features + new v1.3-v1.4 features
- `1.3.0` - Tree/filtering features (v1.3)
- `1.4.0` - Output customization (v1.4)

## Workflow: Adding a Release

### Step 1: Development
- Implement features in feature branches
- Make commits as usual

### Step 2: Pre-Release
1. Update `Cargo.toml` version field manually
   ```toml
   [package]
   version = "1.4.0"  # Update this
   ```

2. Update `TODO.md` to mark completed features
   ```markdown
   ## ✅ v1.4 – Output Customization
   - [x] Feature 1
   - [x] Feature 2
   ```

3. Commit changes:
   ```bash
   git add Cargo.toml TODO.md
   git commit -m "chore: release version 1.4.0"
   ```

### Step 3: Create Release Tag
```bash
git tag -a "v1.4.0" -m "Release v1.4.0: Output customization"
git push origin main
git push origin "v1.4.0"
```

### Step 4: GitHub Actions Automation
- Release workflow triggers on `v*.*.*` tags
- Builds binaries for all platforms
- Creates GitHub release with assets
- Publishes to crates.io (requires `CRATES_TOKEN`)

## Important Notes

### ✅ DO Use Manual Versioning
- Edit `Cargo.toml` directly for version bumps
- Tag releases after committing version changes
- Use semantic versioning consistently

### ❌ DON'T Use GitHub Actions Version Bump
- The `version-bump.yml` workflow is disabled by policy
- Manual control ensures predictable versioning
- Prevents conflicts with feature development

### ⚠️ GitHub Actions Coordination
1. **version-bump.yml**: Manual workflow_dispatch only (currently unused)
2. **release.yml**: Triggered on tags (automated, do not disable)
3. **ci.yml**: Runs on PRs/pushes (automated)

## Release Checklist
- [ ] All features implemented for version
- [ ] Tests passing (`cargo test`)
- [ ] Documentation updated (README, ROADMAP, TODO)
- [ ] Version bumped in `Cargo.toml`
- [ ] Commit with message "chore: release version X.Y.Z"
- [ ] Tag created: `git tag -a "vX.Y.Z" -m "Release vX.Y.Z"`
- [ ] Tag pushed: `git push origin "vX.Y.Z"`
- [ ] GitHub release automatically created
