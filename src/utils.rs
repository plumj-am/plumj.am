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
		div { class: "flex flex-col text-right text-white/40 pr-2 mr-1 border-r-1 border-white/20 select-none min-w-8",
			for line_num in line_numbers {
				if line_num == 1 {
					div { class: "leading-6 h-6 font-bold text-white mr-2",
						"{line_num}"
					}
				} else {
					div { class: "leading-6 h-6",
						"{line_num}"
					}
				}
			}
		}
	}
}
