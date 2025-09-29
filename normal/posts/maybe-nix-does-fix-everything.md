---
title: "Maybe Nix does fix everything."
date: "2025-09-29"
edited: "2025-09-29"
slug: "maybe-nix-does-fix-everything"
brief: "My experience switching from Windows to NixOS."
---

## Windows? Why!?

The big thing holding me back from switching to Linux as a daily driver has
been gaming. I've long been under the impression that Linux is terrible for it,
and despite playing fewer and fewer games, I was never willing to fully give it
up.

For the last year or so, I have been running a Debian server at home as well as
two remote servers running NixOS, and a MacBook running
[nix-darwin](https://github.com/nix-darwin/nix-darwin) for the last month, so
familiarity with Linux was never a blocker.

Anyway, I've been using [NixOS-WSL](https://github.com/nix-community/NixOS-WSL)
for a few months, which has worked quite nicely, and so I was happy to stay in
MS land for a while longer...

## What happened

Everything went wrong about a week ago. I had spent the day working on three
separate projects, and as I was getting ready to make the final commits of the
day, my WSL crashed and I was told that the VHD was corrupted. Thanks to
version control, only about two hours of work was lost, but it was yet another
nail in the MS coffin.

I fixed WSL, forgot the losses, and hoped it was just a stroke of bad luck.

Three days later, it happened again, and another two hours of work was flushed
away.

I gave up and so it began. I instantly adjusted my NixOS config, flashed NixOS
to a USB drive, created a new partition on my hard drive, and installed NixOS. I
then did the same for my laptop. It took, at most, two hours to get up and
running.

I recommend using the GUI installer and following the guide here:
[github/mikeroyal/NixOS-Guide](https://github.com/mikeroyal/NixOS-Guide).

## Pain points

I'll admit that NixOS initially appears to be quite daunting, especially when
you see massive configurations with 4000 lines of Nix. But you don't need so
much, especially to get started!

The documentation can be lacking in places, but I found that
[mynixos.com](https://mynixos.com) helped massively for finding options,
packages, and more.


## How it's going

I've been using NixOS on both my desktop and laptop for the last week, and I
honestly could not be happier.

My game of choice, Overwatch 2, runs exceptionally well, possibly even better
than it did on Windows, achieving a very smooth average of 580 fps with **zero**
stuttering.

Thanks to Proton and Steam's compatibility layer, most games just work!

## Why you should switch

Here's a rapid-fire summary of my thoughts to try to convince anyone I can to
escape the MS trap:

- My environment looks and feels exactly how I want it to.
- Everything is instantaneous.
- Gaming works perfectly, even at demanding frame rates.
- Idle RAM usage with Hyprland, Waybar, and more sits at 1.4 GB (it was at least
  5 GB on Windows with nothing running, by the way).
- My start menu isn't written in React and doesn't invert text colours randomly
  (well actually I don't have one, unless Fuzzel counts).
- Edge, Copilot, and OneDrive aren't forced down my throat.
- I don't need a license.
- No more janky applications that have been ported or must be used via WSL.
- [Hyprland](https://hypr.land/).
- [kitty](https://github.com/kovidgoyal/kitty) terminal.
- Tiling window managers will change your life. However,
  [komorebi](https://github.com/LGUG2Z/komorebi) on Windows is excellent and
  they're doing a great job considering the platform they have to build for.
- WSL is, in reality, not great, and doesn't come close to the real thing.

For NixOS specifically:
- If anything breaks, you can roll back to a previous generation.
- Enormous (the biggest) selection of packages.
- Builds the same, every time, based on your declared configuration.
- Graphics drivers are a breeze and can be handled declaratively in a [few lines
  of configuration](https://github.com/plumj-am/nixos/blob/master/modules/linux/games.nix).
  (No manual driver hunting or hacky workarounds!)
- Personally, I've only had positive experiences with other NixOS users.
  I am aware that parts of the community have a poor reputation for a number of
  ridiculous reasons (it looks like things are changing, though!) but this can
  mostly be ignored. Just steer clear of Bluesky and the official forums for
  discussions.

## Closing thoughts

I'm now all in on Nix and use flakes
([simple](https://github.com/plumj-am/avr-switchbot/blob/master/flake.nix),
[workspaces](https://github.com/plumj-am/plumj.am/blob/master/flake.nix)) for my
projects, [garnix](https://garnix.io) for CI, and, of course, NixOS,
[nix-darwin](https://github.com/nix-darwin/nix-darwin), and
[nixos-anywhere](https://github.com/nix-community/nixos-anywhere) for all my
machines.

While there was a learning curve with Nix's declarative configuration approach,
the investment has paid off tremendously.

I don't find myself missing anything Windows-specific, and even debuggers like
[RAD Debugger](https://github.com/EpicGamesExt/raddebugger) are making their way
to Linux, if that's your thing.

I couldn't recommend it more, especially now that I can see how great it is for
gaming too, which I think holds a lot of people back.

Finally, I'd like to thank RGBCube ([Xitter](https://x.com/HSVSphere),
[GitHub](https://github.com/RGBCube)) - I used his
[NixOS configurations](https://github.com/RGBCube/ncc) as a reference for my
own once I realised I needed a more complex setup for all my hosts.

---

## Related content

My NixOS configurations: [GitHub](https://github.com/plumj-am/nixos), [Forgejo](https://git.plumj.am/plumjam/nixos).

The largest package repository: [search.nixos.org/packages](https://search.nixos.org/packages).

Options, packages, and more: [mynixos.com](https://mynixos.com/).

Mandatory fastfetch:

![NixOS fastfetch output](/assets/maybe-nix-does-fix-everything-1.png)
