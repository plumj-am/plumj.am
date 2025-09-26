#!/usr/bin/env nu

let current_version = (open Cargo.toml | get workspace.package.version)

print $"Current version: ($current_version)"
let new_version = (input "Enter new version: ")

if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+$') {
    print "Error: Version must follow semantic versioning"
    exit 1
}

print $"Updating version from ($current_version) to ($new_version)"

# Update version in Cargo.toml.
open Cargo.toml | upsert workspace.package.version $new_version | save -f Cargo.toml
taplo fmt Cargo.toml

# Build TailwindCSS for both versions.
# Tailwind is in the flake for this project.
print "Building TailwindCSS..."
bunx tailwindcss -i='./nerd/input.css' -o='./nerd/gen-tailwind.css' --minify --content='nerd/**/*.rs'
bunx tailwindcss -i='./normal/input.css' -o='./normal/gen-tailwind.css' --minify --content='normal/**/*.rs'

# Run checks.
print "Running checks..."
cargo check --quiet
dx check --package nerd
dx check --package normal

# Build both versions.
print "Building nerd version..."
dx build --release --package nerd
mkdir nerd_dist
cp --recursive target/dx/nerd/release/web/public/* nerd_dist/
cp nerd_dist/index.html nerd_dist/404.html

print "Building normal version..."
dx build --release --package normal
mkdir normal_dist
cp --recursive target/dx/normal/release/web/public/* normal_dist/
cp normal_dist/index.html normal_dist/404.html

# Deploy to server.
let host = (input "Server host: ")
let user = (input "Server user: ")

if ($host | is-empty) {
    print "Skipping deployment (no host provided)"
} else {
    print "Deploying to server..."

    # Deploy nerd version to /nerd/ subdirectory.
    print "   Deploying nerd version..."
    rsync -avz --delete nerd_dist/ $"($user)@($host):/var/www/site/nerd/"

    # Deploy normal version to root.
    print "   Deploying normal version..."
    rsync -avz --delete normal_dist/ $"($user)@($host):/var/www/site/"
}

# Cleanup.
rm -rf nerd_dist normal_dist

# Commit and tag.
print "Creating release commit..."
jj commit -m $"chore: Release v($new_version)"
jj git export
git tag -f $"v($new_version)" --annotate --message $"v($new_version)"

# Optional: Push changes.
let should_push = (["yes", "no"] | input list "Push to git remote?")
if ($should_push == "yes") {
    jj tug
    jj push
    git push origin $"v($new_version)"
}

print $"Released v($new_version)"
