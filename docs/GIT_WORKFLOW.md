# Git Workflow & Branching Strategy

## 🏗️ Branch Structure

```
main (production-ready)
├── develop (integration branch)
├── feature/xyz (feature branches)
├── hotfix/xyz (urgent fixes)
└── release/v1.0.0 (release preparation)
```

## 📋 Branch Descriptions

### **main**
- **Purpose**: Production-ready code
- **Protection**: Requires PR review
- **Deployment**: Auto-deploys to production
- **Merges**: Only from `develop` or `hotfix/*`

### **develop**
- **Purpose**: Integration branch for features
- **Protection**: Requires PR review
- **Merges**: From `feature/*` branches
- **Deployment**: Auto-deploys to staging

### **feature/***
- **Purpose**: New features and improvements
- **Naming**: `feature/descriptive-name`
- **Examples**: `feature/user-authentication`, `feature/cli-commands`
- **Merges**: Into `develop`

### **hotfix/***
- **Purpose**: Critical production fixes
- **Naming**: `hotfix/issue-description`
- **Examples**: `hotfix/security-vulnerability`, `hotfix/crash-fix`
- **Merges**: Into both `main` and `develop`

### **release/***
- **Purpose**: Release preparation
- **Naming**: `release/v1.0.0`
- **Merges**: Into `main` and `develop`

## 🔄 Workflow Process

### **Feature Development**
```bash
# 1. Start from develop
git checkout develop
git pull origin develop

# 2. Create feature branch
git checkout -b feature/your-feature-name

# 3. Develop and commit
git add .
git commit -m "feat: add new feature"

# 4. Push and create PR
git push --set-upstream origin feature/your-feature-name
# Create PR: feature/your-feature-name → develop
```

### **Hotfix Process**
```bash
# 1. Start from main
git checkout main
git pull origin main

# 2. Create hotfix branch
git checkout -b hotfix/critical-fix

# 3. Fix and commit
git add .
git commit -m "fix: critical production issue"

# 4. Push and create PR
git push --set-upstream origin hotfix/critical-fix
# Create PR: hotfix/critical-fix → main
```

### **Release Process**
```bash
# 1. Start from develop
git checkout develop
git pull origin develop

# 2. Create release branch
git checkout -b release/v1.0.0

# 3. Version bump and final fixes
# Update Cargo.toml version
git add .
git commit -m "chore: bump version to v1.0.0"

# 4. Push and create PRs
git push --set-upstream origin release/v1.0.0
# Create PR: release/v1.0.0 → main
# Create PR: release/v1.0.0 → develop
```

## 🛡️ Branch Protection Rules

### **main**
- ✅ Require pull request reviews
- ✅ Require status checks to pass
- ✅ Require branches to be up to date
- ✅ Restrict pushes
- ✅ Allow force pushes: ❌
- ✅ Allow deletions: ❌

### **develop**
- ✅ Require pull request reviews
- ✅ Require status checks to pass
- ✅ Require branches to be up to date
- ✅ Restrict pushes
- ✅ Allow force pushes: ❌
- ✅ Allow deletions: ❌

## 🤖 Dependabot Integration

### **Dependabot PRs**
- **Target**: `develop` branch
- **Auto-merge**: ✅ (if CI passes)
- **Review required**: ✅
- **CI**: Fast-tracked (8 min timeout)

### **Dependabot Configuration**
```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "sunday"
      time: "04:00"
    target-branch: "develop"
    reviewers:
      - "Leo-Noriega"
```

## 📊 CI/CD Pipeline

### **Feature Branches**
- **CI**: Full suite (quality, tests, coverage)
- **Deployment**: None
- **Duration**: ~15-20 minutes

### **develop**
- **CI**: Full suite + security audit
- **Deployment**: Staging environment
- **Duration**: ~20-25 minutes

### **main**
- **CI**: Full suite + release build
- **Deployment**: Production environment
- **Duration**: ~25-30 minutes

### **Dependabot PRs**
- **CI**: Fast-tracked (check, test, fmt, clippy)
- **Deployment**: None
- **Duration**: ~8-10 minutes

## 🎯 Best Practices

### **Commit Messages**
```bash
# Format: type(scope): description
git commit -m "feat(cli): add user authentication command"
git commit -m "fix(core): resolve memory leak in parser"
git commit -m "docs(readme): update installation instructions"
git commit -m "chore(deps): update dependencies"
```

### **Branch Naming**
```bash
# Features
feature/user-authentication
feature/cli-commands
feature/api-integration

# Hotfixes
hotfix/security-vulnerability
hotfix/crash-fix
hotfix/performance-issue

# Releases
release/v1.0.0
release/v1.1.0
```

### **PR Templates**
- Use GitHub PR templates
- Include description of changes
- Link related issues
- Add screenshots if UI changes
- Request specific reviewers

## 🚀 Quick Commands

```bash
# Start new feature
git checkout develop && git pull && git checkout -b feature/name

# Create hotfix
git checkout main && git pull && git checkout -b hotfix/name

# Create release
git checkout develop && git pull && git checkout -b release/v1.0.0

# Update feature branch
git checkout feature/name && git rebase develop

# Merge develop to main (after release)
git checkout main && git merge develop && git push
``` 