use dioxus::prelude::*;

pub struct SocialLink {
	pub name: &'static str,
	pub url: &'static str,
	pub icon: Asset,
}

pub static SOCIAL_LINKS: &[SocialLink] = &[
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
];

pub struct Me {
	pub name: &'static str,
	pub image: Asset,
	pub age: &'static str,
	pub langs: &'static [&'static str],
	pub frameworks: &'static [&'static str],
}

pub static ME: Me = Me {
	name: "James",
	image: asset!("assets/photo.webp"),
	age: "24",
	langs: &["Rust", "Typescript", "Svelte", "Go"],
	frameworks: &[
		"Dioxus (Rust)",
		"Poem (Rust)",
		"Chi (Go)",
		"Astro (Typescript)",
		"SvelteKit (Svelte)",
		"Next.js (Typescript)",
	],
};
