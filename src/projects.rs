pub struct ProjectInfo {
	pub name: &'static str,
	pub desc: &'static str,
	pub type_of: &'static str,
	pub tech: &'static [&'static str],
	pub gh_url: &'static str,
	pub site_url: &'static str,
}

impl ProjectInfo {
	pub fn tech_str(&self) -> String {
		self.tech.join(", ")
	}
}

pub static PROJECTS: &[ProjectInfo] = &[
	ProjectInfo {
		name: "dr-radka",
		desc: "",
		type_of: "website",
		tech: &["SvelteKit", "Typescript", "Tailwind", "Sanity CMS"],
		gh_url: "N/A",
		site_url: "https://dr-radka.pl",
	},
	ProjectInfo {
		name: "charfreq-rs",
		desc: "",
		type_of: "CLI tool",
		tech: &["Rust"],
		gh_url: "https://github.com/jamesukiyo/charfreq-rs",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "shell-rs",
		desc: "",
		type_of: "CLI tool",
		tech: &["Rust"],
		gh_url: "https://github.com/jamesukiyo/shell-rs",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "windows-setup",
		desc: "",
		type_of: "script",
		tech: &["Bash"],
		gh_url: "https://github.com/jamesukiyo/windows-setup",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "search-this.nvim",
		desc: "",
		type_of: "neovim plugin",
		tech: &["Lua"],
		gh_url: "https://github.com/jamesukiyo/search-this.nvim",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "server-health-monitor",
		desc: "",
		type_of: "utility",
		tech: &["C++", "Python", "Bash"],
		gh_url: "https://github.com/jamesukiyo/server-health-monitor",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "pausarr",
		desc: "",
		type_of: "script",
		tech: &["Bash"],
		gh_url: "https://github.com/jamesukiyo/pausarr",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "nvim",
		desc: "",
		type_of: "config",
		tech: &["Lua"],
		gh_url: "https://github.com/jamesukiyo/nvim",
		site_url: "N/A",
	},
	ProjectInfo {
		name: "me",
		desc: "",
		type_of: "website",
		tech: &["Rust", "Dioxus", "Tailwind"],
		gh_url: "https://github.com/jamesukiyo/me",
		site_url: "https://jamesukiyo.com",
	},
];
