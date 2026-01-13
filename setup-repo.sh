#!/bin/bash

# clock-rand Repository Setup Script
# This script initializes the Git repository with proper branching structure

set -e

echo "🚀 Setting up clock-rand repository..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in clock-rand project directory"
    exit 1
fi

# Initialize git if not already done
if [ ! -d ".git" ]; then
    echo "📝 Initializing Git repository..."
    git init
fi

# Add all files
echo "📦 Staging all files..."
git add .

# Initial commit
echo "💾 Creating initial commit..."
git commit -m "Initial commit: clock-rand v1.0.0

- Production-ready Rust RNG crate
- Cryptographically secure RNGs (ChaCha20, Blake3-DRBG, AES-CTR)
- Fast deterministic RNGs (Xoshiro256+, PCG64)
- Blockchain-aware RNGs with fork detection
- Comprehensive testing and security audits
- Professional CI/CD and documentation"

# Create develop branch
echo "🌿 Creating develop branch..."
git checkout -b develop

echo "✅ Repository setup complete!"
echo ""
echo "📋 Next steps:"
echo "1. Create public repository on GitHub: https://github.com/Olyntar-Labs/clock-rand"
echo "2. Add remote: git remote add origin https://github.com/Olyntar-Labs/clock-rand.git"
echo "3. Push branches: git push -u origin main develop"
echo "4. Set up branch protection rules in GitHub Settings"
echo "5. Configure repository topics and description"
echo ""
echo "🔗 Repository URL: https://github.com/Olyntar-Labs/clock-rand"
echo "📖 See BRANCHING.md for complete branching strategy"