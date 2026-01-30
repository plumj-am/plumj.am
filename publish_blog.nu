#!/usr/bin/env nu
# nu-lint-ignore: max_function_body_length
def main [
   --update: string # The blog post to update.
] {
   let variants = [ nerd normal ]

   print "Building TailwindCSS..."
   for v in $variants {
      bunx tailwindcss --input=$"./($v)/input.css" --output=$"./($v)/gen-tailwind.css" --minify --content=$"($v)/**/*.rs" --quiet
   }

   print "Running checks..."
   cargo check --quiet # nu-lint-ignore: wrap_external_with_complete
   for v in $variants {
      dx check --package $v
   }

   for v in $variants {
      print $"Building ($v) version..."
      dx build --release --package $v
      try {
         mkdir $"($v)_dist"
         cp --recursive ($"./target/dx/($v)/release/web/public/*" | into glob) $"($v)_dist/"
         cp $"($v)_dist/index.html" $"($v)_dist/404.html"
      } catch {|e|
         print $"Failed during ($v) post-build: ($e.msg)"
      }
   }

   print "Deploying to server..."
   print "   Deploying normal version..."
   rsync --archive --compress --verbose --delete normal_dist/ $"root@plum:/var/www/site/"
   print "   Deploying nerd version..."
   rsync --archive --compress --verbose --delete nerd_dist/ $"root@plum:/var/www/site/nerd/"

   # Cleanup.
   try {
      rm --recursive --force normal_dist
      rm --recursive --force nerd_dist
   }

   let can_update = ($update != null and ($update | is-not-empty))
   mut action = ""
   if ($can_update) {
      print "Creating update commit..."
      jj commit -m $"blog.update: Update post \"($update)\"." # nu-lint-ignore: check_typed_flag_before_use
      $action = "Updated"
   } else {
      print "Creating publish commit..."
      jj commit -m $"blog.publish: New post \((date now | format date %Y-%m-%d)\)."
      $action = "Published"
   }

   let should_push = ([ yes no ] | input list "Push to git remote?")
   if ($should_push == yes) {
      jj tug
      jj push
   }

   print $"($action) a blog post."
}
