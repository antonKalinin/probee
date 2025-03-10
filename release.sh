#!/bin/bash

# Check if argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 [minor|patch]"
    exit 1
fi

# Check if argument is valid
if [ "$1" != "minor" ] && [ "$1" != "patch" ]; then
    echo "Error: Argument must be either 'minor' or 'patch'"
    exit 1
fi

# Store the bump type
BUMP_TYPE=$1

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed"
    exit 1
fi

# Check if cargo-edit is installed
if ! cargo install --list | grep -q "cargo-edit"; then
    echo "Error: cargo-edit is not installed. Installing now..."
    cargo install cargo-edit
    if [ $? -ne 0 ]; then
        echo "Error: Failed to install cargo-edit"
        exit 1
    fi
fi

# Step 1: Bump version in Cargo.toml
echo "Bumping $BUMP_TYPE version..."
cargo bump $BUMP_TYPE
if [ $? -ne 0 ]; then
    echo "Error: Failed to bump version"
    exit 1
fi

# Extract the new version number from Cargo.toml
VERSION=$(grep -m 1 'version = "[0-9]*\.[0-9]*\.[0-9]*"' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    echo "Error: Failed to extract version from Cargo.toml"
    exit 1
fi

echo "New version: $VERSION"

# Step 2: Git commit with version as message
echo "Committing changes..."
git add Cargo.toml Cargo.lock
git commit -m "v$VERSION"
if [ $? -ne 0 ]; then
    echo "Error: Failed to commit changes"
    exit 1
fi

# Step 3: Create git tag
echo "Creating tag v$VERSION..."
git tag "v$VERSION"
if [ $? -ne 0 ]; then
    echo "Error: Failed to create tag"
    exit 1
fi

# Step 4: Push changes to remote
echo "Pushing changes to remote..."
git push
if [ $? -ne 0 ]; then
    echo "Error: Failed to push changes"
    exit 1
fi

# Step 5: Push tags to remote
echo "Pushing tags to remote..."
git push --tags
if [ $? -ne 0 ]; then
    echo "Error: Failed to push tags"
    exit 1
fi

echo "Successfully bumped version to $VERSION and pushed changes"
