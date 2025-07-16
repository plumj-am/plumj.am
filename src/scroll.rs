use dioxus::prelude::*;

const LINE_HEIGHT: f64 = 24.0;

pub fn use_scroll_snapping() {
	use_effect(|| {
		spawn(async move {
			let _eval = document::eval(&format!(
				r"
				(() => {{
					let timeout;
					window.addEventListener('scroll', () => {{
						clearTimeout(timeout);
						timeout = setTimeout(() => {{
							const scroll = window.pageYOffset;
							const target = Math.round(scroll / {LINE_HEIGHT}) * {LINE_HEIGHT};
							if (Math.abs(scroll - target) > 1) {{
								window.scrollTo({{ top: target, behavior: 'auto' }});
							}}
						}}, 10);
					}}, {{ passive: true }});
				}})();
				"
			));
		});
	});
}
