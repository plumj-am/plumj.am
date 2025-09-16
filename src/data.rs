use dioxus::prelude::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

//
// ME
//
pub struct SocialLink {
	pub name: &'static str,
	pub url: &'static str,
	pub icon: &'static str,
}

pub struct Me {
	pub name: &'static str,
	pub image: Asset,
	pub email: &'static str,
	pub location: &'static str,
	pub langs: &'static [&'static str],
	pub scripting: &'static [&'static str],
	pub frameworks: &'static [&'static str],
	pub tools: &'static [&'static str],
	pub socials: &'static [SocialLink],
}

pub static ME: Me = Me {
	name: "PlumJam",
	image: asset!("assets/photo.png"),
	email: "contact@plumj.am",
	location: "Poland",
	langs: &["Rust", "Typescript", "Svelte", "Go", "Lua"],
	scripting: &["Nushell", "Bash", "Python"],
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
			icon: "fa-brands fa-github",
		},
		SocialLink {
			name: "Xitter",
			url: "https://x.com/plumj_am",
			icon: "fa-brands fa-twitter",
		},
	],
};

//
// PROJECTS
//
pub enum ProjectType {
	Website,
	CliTool,
	Script,
	Plugin,
	Utility,
	Config,
	WebApp,
}

impl ProjectType {
	pub const fn as_str(&self) -> &'static str {
		match self {
			Self::Website => "website",
			Self::CliTool => "CLI tool",
			Self::Script => "script",
			Self::Plugin => "plugin",
			Self::Utility => "utility",
			Self::Config => "config",
			Self::WebApp => "web application",
		}
	}
}

impl std::fmt::Display for ProjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

pub struct ProjectInfo {
	pub name: &'static str,
	pub short_desc: &'static str,
	pub long_desc: &'static str,
	pub project_type: ProjectType,
	pub tech_used: &'static [&'static str],
	pub github_url: Option<&'static str>,
	pub site_url: Option<&'static str>,
}

impl ProjectInfo {
	pub fn tech_used_str(&self) -> String {
		self.tech_used.join(", ")
	}
	pub fn github_url_str(&self) -> &str {
		self.github_url.unwrap_or("N/A")
	}
	pub fn site_url_str(&self) -> &str {
		self.site_url.unwrap_or("N/A")
	}
	pub fn main_tech_used(&self) -> &str {
		self.tech_used[0]
	}
}

pub static PROJECTS: &[ProjectInfo] = &[
	ProjectInfo {
		name: "plumj.am",
		short_desc: "This website!",
		long_desc: "",
		project_type: ProjectType::Website,
		tech_used: &["Rust", "Dioxus", "Tailwind"],
		github_url: Some("https://github.com/jamesukiyo/jamesukiyo.github.io"),
		site_url: Some("https://jamesukiyo.github.io"),
	},
	ProjectInfo {
		name: "Charfreq",
		short_desc: "The fastest character counter.",
		long_desc: "",
		project_type: ProjectType::CliTool,
		tech_used: &["Rust"],
		github_url: Some("https://github.com/jamesukiyo/charfreq-rs"),
		site_url: Some("https://crates.io/charfreq"),
	},
	ProjectInfo {
		name: "Sunny",
		short_desc: "Check the weather from your terminal!",
		long_desc: "",
		project_type: ProjectType::CliTool,
		tech_used: &["Rust"],
		github_url: Some("https://github.com/jamesukiyo/sunny-rs"),
		site_url: Some("https://crates.io/sunny-cli"),
	},
	ProjectInfo {
		name: "NixOS",
		short_desc: "PlumJam's NixOS configuration collection",
		long_desc: "",
		project_type: ProjectType::Config,
		tech_used: &["Nix"],
		github_url: Some("https://github.com/jamesukiyo/nixos"),
		site_url: None,
	},
	ProjectInfo {
		name: "Dr. Radka",
		short_desc: "Client website.",
		long_desc: "",
		project_type: ProjectType::Website,
		tech_used: &["SvelteKit", "Typescript", "Tailwind", "Sanity CMS"],
		github_url: None,
		site_url: Some("https://dr-radka.pl"),
	},
	ProjectInfo {
		name: "Jash",
		short_desc: "A basic shell.",
		long_desc: "",
		project_type: ProjectType::CliTool,
		tech_used: &["Rust"],
		github_url: Some("https://github.com/jamesukiyo/jash-rs"),
		site_url: None,
	},
	ProjectInfo {
		name: "Windows Setup",
		short_desc: "Up and running with NixOS-WSL in minutes.",
		long_desc: "",
		project_type: ProjectType::Script,
		tech_used: &["Bash"],
		github_url: Some("https://github.com/jamesukiyo/windows-setup"),
		site_url: None,
	},
	ProjectInfo {
		name: "Search This",
		short_desc: "Initiate a web search from NeoVim.",
		long_desc: "",
		project_type: ProjectType::Plugin,
		tech_used: &["Lua"],
		github_url: Some("https://github.com/jamesukiyo/search-this.nvim"),
		site_url: None,
	},
	ProjectInfo {
		name: "Server Health Monitor",
		short_desc: "IOT project for local monitoring.",
		long_desc: "",
		project_type: ProjectType::Utility,
		tech_used: &["C++", "Python", "Bash"],
		github_url: Some("https://github.com/jamesukiyo/server-health-monitor"),
		site_url: None,
	},
	ProjectInfo {
		name: "Pausarr",
		short_desc: "A companion script for Tdarr and Jellyfin.",
		long_desc: "",
		project_type: ProjectType::Script,
		tech_used: &["Bash"],
		github_url: Some("https://github.com/jamesukiyo/pausarr"),
		site_url: None,
	},
	ProjectInfo {
		name: "NeoVim",
		short_desc: "(Old) NeoVim configuration files.",
		long_desc: "",
		project_type: ProjectType::Config,
		tech_used: &["Lua"],
		github_url: Some("https://github.com/jamesukiyo/nvim"),
		site_url: None,
	},
	ProjectInfo {
		name: "Nushell",
		short_desc: "(Old) Nushell configuration files.",
		long_desc: "",
		project_type: ProjectType::Config,
		tech_used: &["Nu", "Nushell"],
		github_url: Some("https://github.com/jamesukiyo/nushell"),
		site_url: None,
	},
	ProjectInfo {
		name: "Gocial",
		short_desc: "The forever unfinished social platform.",
		long_desc: "",
		project_type: ProjectType::WebApp,
		tech_used: &["Go"],
		github_url: Some("https://github.com/jamesukiyo/gocial"),
		site_url: None,
	},
	ProjectInfo {
		name: "Prepare Release",
		short_desc: "Automatic version bumping and tagging.",
		long_desc: "",
		project_type: ProjectType::Script,
		tech_used: &["Nushell", "Bash"],
		github_url: Some("https://github.com/jamesukiyo/prepare_release"),
		site_url: None,
	},
	ProjectInfo {
		name: "QuickSnip",
		short_desc: "Instantly create snippets for Vim on the fly.",
		long_desc: "",
		project_type: ProjectType::Plugin,
		tech_used: &["Vimscript"],
		github_url: Some("https://github.com/jamesukiyo/quicksnip.vim"),
		site_url: None,
	},
];
