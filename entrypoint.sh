#!/bin/bash
set -e

REPO_URL="$1"

if [ -z "$REPO_URL" ]; then
  echo "[ERROR] No Git repository URL provided"
  echo "Usage: docker run <image> <git-repo-url>"
  exit 1
fi

echo "[INFO] Cloning repository: $REPO_URL"
git clone "$REPO_URL" repo
cd repo

# Detect and build
if [ -f "foundry.toml" ]; then
  echo "[INFO] Found Foundry project - installing dependencies and building"
  forge install
  forge build
elif [ -f "hardhat.config.js" ] || [ -f "hardhat.config.ts" ]; then
  echo "[INFO] Found Hardhat project - installing Hardhat and building"
  npm install -g hardhat
  npm install
  npx hardhat compile
else
  echo "[INFO] No build system detected - skipping build"
fi

echo "[INFO] Repository ready for Slither analysis"
