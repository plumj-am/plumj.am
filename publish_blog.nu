#!/usr/bin/env nu
print "Building normal version with new blog post..."
dx build --release --package normal
mkdir normal_dist
cp --recursive target/dx/normal/release/web/public/* normal_dist/


# Deploy to server.
let host = (input "Server host: ")
let user = (input "Server user: ")

if ($host | is-empty) {
    print "Skipping deployment (no host provided)"
} else {
    print "Deploying to server..."

    # Deploy normal version to root and only overwrite changed files.
    print "   Deploying normal version..."
    rsync -avz --checksum normal_dist/ $"($user)@($host):/var/www/site/"
}

# Cleanup.
rm -rf normal_dist

# Commit and tag.
print "Creating publish commit..."
jj commit -m $"publish: New blog post ((date now | format date "%Y-%m-%d"))."

# Optional: Push changes.
let should_push = (["yes", "no"] | input list "Push to git remote?")
if ($should_push == "yes") {
    jj tug
    jj push
}

print "Published a new blog post."
