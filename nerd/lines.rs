//! Editor-style decorations: line gutters and highlight bars.

use topcoat::{
    Result,
    view::{
        View,
        ViewExt as _,
        component,
        view,
    },
};

/// The kind of text rendered on a line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineType {
    H1,
    H2,
    P,
    #[default]
    Blank,
}

/// A single line of the "file" being edited.
///
/// `classes` applies to the wrapped element, and `text` is its content.
#[component]
pub async fn line(
    #[default] type_of: LineType,
    #[default] text: String,
    #[default] classes: &'static str,
) -> Result<impl View> {
    let content = match type_of {
        LineType::H1 => view! { <h1 class=(classes)>(text)</h1> }.boxed(),
        LineType::H2 => view! { <h2 class=(classes)>(text)</h2> }.boxed(),
        LineType::P => view! { <p class=(classes)>(text)</p> }.boxed(),
        LineType::Blank => view! { <span></span> }.boxed(),
    };

    Ok(view! { <div class="line-content">(content)</div> })
}

/// Vertical strip of line numbers, like an editor's sign column.
#[component]
pub async fn line_numbers(#[default(50_i32)] max_lines: i32) -> Result<impl View> {
    let line_numbers: Vec<i32> = (1_i32..=max_lines).collect();

    Ok(view! {
        <div class="flex flex-col text-right text-base text-white/40 pr-2 mr-1 border-r-1 border-white/20 select-none min-w-8 relative">
            hi_line_nr()
            for line_num in line_numbers {
                <div class="leading-6 h-6 text-shadow-md text-shadow-[#0f1116]">(line_num)</div>
            }
        </div>
    })
}

/// Highlights the first line number of [`line_numbers`].
#[component]
pub async fn hi_line_nr() -> Result<impl View> {
    Ok(view! {
        <div class="fixed z-3 text-right pr-2 mr-8 min-w-8 h-[24px] mix-blend-difference"></div>
    })
}

/// Highlights the first line of the page.
#[component]
pub async fn hi_line() -> Result<impl View> {
    Ok(view! {
        <div class="flex fixed w-[calc(var(--container-6xl)-2px)] h-6 bg-white/20 mx-auto justify-center"></div>
    })
}

/// Marks the cursor position at (1, 1).
#[component]
pub async fn hi_cursor() -> Result<impl View> {
    Ok(view! {
        <div class="fixed w-[12px] h-6 bg-[#F2EEEB] text-right pr-[1px] ml-[-3px] justify-center z-2 animate-blink mix-blend-difference"></div>
    })
}
