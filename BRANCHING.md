# Branching Strategy for clock-rand

This document outlines the branching strategy and workflow for the clock-rand Rust crate.

## 🌳 Branch Structure

### Permanent Branches

#### `main` (Production)
- **Purpose**: Production-ready, stable releases
- **Protection**: Highest protection level
- **Merges**: Only from `release/*` branches or hotfixes
- **Version**: Always reflects the latest stable version

#### `develop` (Integration)
- **Purpose**: Integration branch for features and releases
- **Protection**: Medium protection level
- **Merges**: Feature branches and bug fixes
- **Status**: Latest development state

### Temporary Branches

#### `feature/*` (Features)
- **Naming**: `feature/description-of-feature`
- **Purpose**: New features, enhancements
- **Base**: `develop`
- **Merge**: Back to `develop` via PR

#### `bugfix/*` (Bug Fixes)
- **Naming**: `bugfix/issue-description`
- **Purpose**: Bug fixes for develop
- **Base**: `develop`
- **Merge**: Back to `develop` via PR

#### `hotfix/*` (Hotfixes)
- **Naming**: `hotfix/critical-issue`
- **Purpose**: Critical fixes for production
- **Base**: `main`
- **Merge**: To both `main` and `develop` via PR

#### `release/*` (Releases)
- **Naming**: `release/v1.2.3`
- **Purpose**: Release preparation and testing
- **Base**: `develop`
- **Merge**: To `main` and back to `develop` via PR

## 🚀 Workflow

### 1. Development Setup

```bash
# Initialize repository
git init
git add .
git commit -m "Initial commit: clock-rand v1.0.0"

# Create develop branch
git checkout -b develop
git push -u origin develop

# Protect main branch (set up in GitHub UI)
# Protect develop branch (set up in GitHub UI)
```

### 2. Feature Development

```bash
# Start new feature
git checkout develop
git pull origin develop
git checkout -b feature/add-new-rng-algorithm

# Work on feature, commit regularly
git add .
git commit -m "feat: implement new RNG algorithm"

# Push and create PR
git push -u origin feature/add-new-rng-algorithm
# Create PR to merge into develop
```

### 3. Release Process

```bash
# Create release branch from develop
git checkout develop
git pull origin develop
git checkout -b release/v1.1.0

# Final testing and version bump
# Update version in Cargo.toml
git add Cargo.toml
git commit -m "chore: bump version to 1.1.0"

# Merge release to main
git checkout main
git merge release/v1.1.0 --no-ff -m "release: merge v1.1.0 to main"

# Tag the release
git tag -a v1.1.0 -m "Release version 1.1.0"
git push origin main --tags

# Merge back to develop
git checkout develop
git merge release/v1.1.0 --no-ff -m "chore: merge release back to develop"

# Clean up release branch
git branch -d release/v1.1.0
git push origin --delete release/v1.1.0
```

### 4. Hotfix Process

```bash
# Create hotfix from main
git checkout main
git pull origin main
git checkout -b hotfix/critical-security-fix

# Fix the issue
git add .
git commit -m "fix: critical security vulnerability in RNG"

# Merge to main
git checkout main
git merge hotfix/critical-security-fix --no-ff

# Tag as patch release
git tag -a v1.0.1 -m "Hotfix: security patch"
git push origin main --tags

# Merge back to develop
git checkout develop
git merge hotfix/critical-security-fix --no-ff
git push origin develop

# Clean up
git branch -d hotfix/critical-security-fix
git push origin --delete hotfix/critical-security-fix
```

## 🔒 Branch Protection Rules

### Main Branch (`main`)
**Required for merge:**
- ✅ Pull request required (1+ approvals)
- ✅ Status checks: `test`, `security`, `clippy`, `fmt-check`
- ✅ Up-to-date branches required
- ✅ Linear history required
- ❌ Force pushes blocked

### Develop Branch (`develop`)
**Required for merge:**
- ✅ Pull request required (1 approval)
- ✅ Status checks: `test`, `clippy`
- ✅ Up-to-date branches required
- ❌ Force pushes blocked

### Feature Branches (`feature/*`)
**Guidelines:**
- ✅ Regular pushes to origin
- ✅ Small, focused changes
- ✅ Descriptive commit messages
- ✅ Rebase before merging

## 📝 Commit Message Convention

Use conventional commits:

```bash
# Features
feat: add ChaCha20 RNG implementation

# Bug fixes
fix: resolve overflow in seed generation

# Documentation
docs: update API documentation for new features

# Testing
test: add property tests for RNG distributions

# Maintenance
chore: update dependencies
refactor: simplify error handling
```

## 🏷️ Version Tags

Tags follow semantic versioning:
- `v1.0.0` - Major releases
- `v1.1.0` - Minor releases
- `v1.1.1` - Patch releases
- `v1.1.0-rc.1` - Release candidates

## 🔄 Syncing Branches

```bash
# Keep branches up to date
git checkout develop
git pull origin develop

git checkout feature/my-feature
git rebase develop  # Or merge if preferred

# Resolve conflicts if any
git add .
git rebase --continue
```

## 🆘 Emergency Procedures

For critical security issues:
1. Create hotfix branch from `main`
2. Implement fix with minimal changes
3. Get security team approval
4. Merge to `main` and tag immediately
5. Backport to `develop`

## 📊 Branch Status Dashboard

Monitor branch health:
- ✅ `main`: Always deployable
- ✅ `develop`: Latest development
- 📊 Feature branches: In active development
- 🚨 Long-lived branches: Review and merge/cleanup

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.