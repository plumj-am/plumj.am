#!/usr/bin/env nu

def date-version [] {
   date now
   | format date "%y-%-m-%-d"
   | str replace --all '-' '.'
}

def main [] {
   # If the current date e.g. `26.4.2.0` is current version, bump to `26.4.2.1`.
   let current_v = open Cargo.toml | get workspace.metadata.version
   let current_minor_v = $current_v | split row '.' | last | into int

   let today = date-version
   let today_part = $current_v | split row '.' | first 3 | str join '.'

   let new_minor_v = if $today_part == $today { $current_minor_v + 1 } else { 0 }
   let new_v = $"(date-version).($new_minor_v)"

   print $"Current version:      ($current_v)"
   print $"Proposed new version: ($new_v)"

   let v_ok = ["yes", "no"] | input list "Is the proposed version correct?"
   let new_v = if $v_ok != "yes" {
      (input "What is the correct new version number? ")
   } else { $new_v }

   if not ($new_v =~ '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$') {
      print --stderr "Error: Version must follow Y.M.D.X calendar versioning
Where:
   Y = Year
   M = Month
   D = Day
   X = Incremented from 0 to allow multiple same-day releases
For example:
   26.4.2.0
   26.4.2.1"
      exit 1
   }

   print $"Updating version from ($current_v) to ($new_v)"

   open Cargo.toml | upsert workspace.metadata.version $new_v | save -f Cargo.toml
   taplo fmt Cargo.toml

   print "Collecting deployment information..."
   let host = input "Server host: "
   let user = input "Server user: "

   if ($host | is-empty) or ($user | is-empty) {
      print --stderr "No server configured, deployment can't complete. Exiting."
      exit 1
   }

   print "Building TailwindCSS..."
   bunx tailwindcss -i='./nerd/input.css' -o='./nerd/gen-tailwind.css' --minify --content='nerd/**/*.rs'
   bunx tailwindcss -i='./normal/input.css' -o='./normal/gen-tailwind.css' --minify --content='normal/**/*.rs'

   print "Running checks..."
   cargo check --quiet
   dx check --package nerd
   dx check --package normal

   # Build and deploy both versions.
   # Nerd must be last to prevent the normal deployment from deleting the sub-directory.
   for p in ["normal", "nerd"] {
      print $"Building ($p) version..."
      dx build --release --package $p
      mkdir $"($p)_dist"
      cp --recursive target/dx/($p)/release/web/public/* ($p)_dist/
      cp ($p)_dist/index.html ($p)_dist/404.html

      let dir = if $p == "nerd" { "nerd/" } else { "" }
      print $"   Deploying ($p) version..."
      try {
         rsync -avz --delete ($p)_dist/ ($user)@($host):/var/www/site/($dir)
      } catch {|e|
         print --stderr $"Error: Deployment failed: ($e.msg)"
         exit 1
      }

      rm --recursive --force ($p)_dist
   }

   print "Creating release change..."
   jj commit -m $"release: `v($new_v)`."

   print "Finished release process!"
   print "Adjust the change and push it to git host."
}
