use dioxus::prelude::*;

#[derive(PartialEq, Clone, Debug, Default)]
pub enum LineType {
	H1,
	H2,
	P,
	#[default]
	Blank,
}

#[derive(PartialEq, Props, Clone)]
pub struct LineProps {
	pub type_of: Option<LineType>,
	pub text: Option<String>,
	pub classes: Option<String>,
}

#[allow(non_snake_case, clippy::needless_pass_by_value)]
pub fn Line(line_props: LineProps) -> Element {
	let text = line_props.text.clone().unwrap_or_default();
	let classes = line_props.classes.clone().unwrap_or_default();
	rsx! {
		div { class: "line-content",
			match line_props.type_of {
				Some(LineType::H1) => rsx!(h1 { class:"{classes}", "{text}" }),
				Some(LineType::H2) => rsx!(h2 { class:"{classes}", "{text}" }),
				Some(LineType::P)  => rsx!(p  { class:"{classes}", "{text}" }),
				_ => rsx!(span {}),
			}
		}
	}
}

#[component]
pub fn LineNumbers(max_lines: Option<i32>) -> Element {
	let line_count = max_lines.unwrap_or(50);
	let line_numbers: Vec<i32> = (1..=line_count).collect();

	rsx! {
		div { class: "flex flex-col text-right text-white/40 pr-2 mr-1 border-r-1 border-white/20 select-none min-w-8 relative",
			HiLineNr {}
			for line_num in line_numbers {
				div { class: "leading-6 h-6 text-shadow-md text-shadow-[#0f1116]",
					"{line_num}"
				}
			}
		}
	}
}

// LINE NUMBER HIGHLIGHTER
#[component]
pub fn HiLineNr() -> Element {
	rsx! {
		div {
			class: "fixed z-3 text-right pr-2 mr-8 min-w-8 h-[24px] mix-blend-difference",
		}
	}
}
// HIGHLIGHTED LINE
#[component]
pub fn HiLine() -> Element {
	rsx! {
		div { class: "flex fixed w-[calc(var(--container-6xl)-2px)] h-6
				bg-white/20 mx-auto justify-center"
		}
	}
}

// HIGHLIGHTED CURSOR
#[component]
pub fn HiCursor() -> Element {
	rsx! {
		div { class: "fixed w-[12px] h-6 bg-[#F2EEEB] text-right
				pr-[1px] ml-[-3px] justify-center z-2 animate-blink mix-blend-difference",
		}
	}
}
