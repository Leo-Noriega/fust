# CI/CD Optimization for GitHub Free Tier

## GitHub Actions Free Tier Limits

- **2,000 minutes/month** for public repositories
- **500 minutes/month** for private repositories
- **6 concurrent jobs** maximum
- **6 hours timeout** per job

## Our Optimization Strategy

### 🎯 **Job Consolidation**
Instead of separate jobs for check, fmt, clippy, and security, we combined them into a single `quality` job to save minutes.

### ⏱️ **Smart Triggers**
- **Coverage**: Only runs on PRs (not every push)
- **Release Build**: Only runs on main branch
- **Tests**: Reduced from 3 platforms to 2 (removed Windows)

### 🚀 **Performance Optimizations**
- **Caching**: Rust cache to speed up builds
- **Timeouts**: 10-15 minute limits per job
- **Components**: Install rustfmt/clippy in same step as toolchain

### 📊 **Estimated Usage per Push/PR**

| Job | Duration | Frequency | Monthly Impact |
|-----|----------|-----------|----------------|
| Quality | ~8 min | Every push/PR | ~240 min |
| Test (Ubuntu) | ~5 min | Every push/PR | ~150 min |
| Test (macOS) | ~8 min | Every push/PR | ~240 min |
| Coverage | ~6 min | PRs only | ~60 min |
| Release Build | ~4 min | Main only | ~20 min |

**Total per push**: ~21 minutes  
**Monthly estimate**: ~710 minutes (35% of free tier)

### 🔧 **Further Optimizations Available**

If you approach the limit:

1. **Remove macOS tests** (save ~240 min/month)
2. **Remove coverage** (save ~60 min/month)  
3. **Use self-hosted runners** (unlimited minutes)
4. **Upgrade to GitHub Pro** ($4/month, 3,000 min)

### 📈 **Monitoring Usage**

Check your usage at: `https://github.com/settings/billing` 