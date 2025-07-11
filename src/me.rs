use dioxus::prelude::*;

pub struct SocialLink {
	pub name: &'static str,
	pub url: &'static str,
	pub icon: Asset,
}

pub struct Me {
	pub name: &'static str,
	pub image: Asset,
	pub age: &'static str,
	pub location: &'static str,
	pub langs: &'static [&'static str],
	pub scripting: &'static [&'static str],
	pub frameworks: &'static [&'static str],
	pub tools: &'static [&'static str],
	pub socials: &'static [SocialLink],
}

pub static ME: Me = Me {
	name: "James",
	image: asset!("assets/photo.webp"),
	age: "24",
	location: "Wrocław, Poland",
	langs: &["Rust", "Typescript", "Svelte", "Go", "Lua"],
	scripting: &["Nushell", "Bash"],
	frameworks: &[
		"Dioxus (Rust)",
		"Poem (Rust)",
		"Chi (Go)",
		"Astro (Typescript)",
		"SvelteKit (Svelte)",
		"Next.js (Typescript)",
	],
	tools: &["Docker", "GitHub Actions", "K3s"],
	socials: &[
		SocialLink {
			name: "GitHub",
			url: "https://github.com/jamesukiyo",
			icon: asset!("/assets/github.svg"),
		},
		SocialLink {
			name: "X/Twitter",
			url: "https://twitter.com/jamesukiyo",
			icon: asset!("/assets/x.svg"),
		},
	],
};
