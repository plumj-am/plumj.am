pub struct Me {
	pub name: &'static str,
	pub age: &'static str,
	pub langs: &'static [&'static str],
	pub frameworks: &'static [&'static str],
}

pub static ME: Me = Me {
	name: "James",
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
