# Branch Management Quick Reference

## 🚀 Quick Start

```bash
# Run setup script
./setup-repo.sh

# Add GitHub remote
git remote add origin https://github.com/clockinchain/clock-rand.git

# Push initial branches
git push -u origin main develop
```

## 🌿 Feature Development

```bash
# Start new feature
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name

# Regular workflow
git add .
git commit -m "feat: description of changes"
git push -u origin feature/your-feature-name

# Create PR to develop branch
# After approval and merge:
git checkout develop
git pull origin develop
git branch -d feature/your-feature-name
git push origin --delete feature/your-feature-name
```

## 🐛 Bug Fixes

```bash
# For develop branch bugs
git checkout develop
git pull origin develop
git checkout -b bugfix/issue-description

# For production bugs (hotfix)
git checkout main
git pull origin main
git checkout -b hotfix/critical-fix
```

## 📦 Releases

```bash
# Prepare release
git checkout develop
git pull origin develop
git checkout -b release/v1.1.0

# Update version in Cargo.toml
# Final testing, then:
git checkout main
git merge release/v1.1.0 --no-ff -m "release: v1.1.0"
git tag -a v1.1.0 -m "Release v1.1.0"
git push origin main --tags

# Merge back to develop
git checkout develop
git merge release/v1.1.0 --no-ff
git push origin develop

# Cleanup
git branch -d release/v1.1.0
git push origin --delete release/v1.1.0
```

## 🔄 Sync & Rebase

```bash
# Update feature branch with latest develop
git checkout feature/your-feature
git fetch origin
git rebase origin/develop

# Resolve conflicts if any
git add .
git rebase --continue

# Force push (since history changed)
git push origin feature/your-feature --force-with-lease
```

## 🧹 Cleanup

```bash
# Delete merged branches locally
git branch -d feature/completed-feature

# Delete remote branches
git push origin --delete feature/completed-feature

# Clean up old branches
git fetch --prune
```

## 📊 Status Checks

```bash
# Check branch status
git status
git log --oneline -5

# Check remote branches
git branch -r

# Check tracking branches
git branch -vv
```

## 🆘 Emergency Commands

```bash
# Abort rebase
git rebase --abort

# Reset to previous commit (BE CAREFUL)
git reset --hard HEAD~1

# Recover lost commits
git reflog
git checkout <commit-hash>
```
