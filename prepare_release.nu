#!/usr/bin/env nu

let current_version = (open Cargo.toml | get workspace.package.version)

print $"Current version: ($current_version)"
let new_version = (input "Enter new version: ")

if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+$') {
    print "Error: Version must follow semantic versioning"
    exit 1
}

let existing_tags = (git tag -l | lines)
if ($existing_tags | any {|tag| $tag == $"v($new_version)"}) {
    print $"Error: Tag v($new_version) already exists"
    exit 1
}

print $"Updating version from ($current_version) to ($new_version)"

let cargo_content = (open Cargo.toml --raw | str replace $'version = "($current_version)"' $'version = "($new_version)"')
$cargo_content | save -f Cargo.toml

cargo check --quiet

# Use jj for commit operations
jj commit -m $"chore: release v($new_version)"

# Export to git and create tag with git
jj git export
git tag $"v($new_version)" --annotate --message $"v($new_version)"

# Push both the commit and tag
git push origin master
git push origin $"v($new_version)"

print $"Released v($new_version)"
