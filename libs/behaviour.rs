//! Browser behaviour shared by both sites.

/// The click handler for the copy-email button.
///
/// Expects an element with `id="copy-email"` carrying the address in
/// `data-email`. Writes it to the clipboard and shows a short toast.
pub const COPY_EMAIL: &str = r##"
	const copy = document.getElementById("copy-email");
	if (copy) {
		copy.addEventListener("click", () => {
			navigator.clipboard.writeText(copy.dataset.email || "").catch(() => {});
			const existing = document.querySelector(".copy-toast");
			if (existing) existing.remove();

			const toast = document.createElement("div");
			toast.className = "copy-toast";
			toast.setAttribute("role", "status");
			toast.setAttribute("aria-live", "polite");
			toast.textContent = "Email copied to clipboard";
			Object.assign(toast.style, {
				position: "fixed", bottom: "1.5rem", right: "1.5rem", padding: "0.35rem 0.75rem",
				borderRadius: "9999px", fontSize: "0.75rem", fontFamily: "inherit",
				background: "rgba(0, 0, 0, 0.75)", color: "#f2eeeb",
				boxShadow: "0 4px 14px rgba(0, 0, 0, 0.25)", zIndex: "9999",
				opacity: "0", transition: "opacity 150ms ease-in-out", pointerEvents: "none",
			});
			document.body.appendChild(toast);
			requestAnimationFrame(() => { toast.style.opacity = "1"; });
			setTimeout(() => {
				toast.style.opacity = "0";
				setTimeout(() => toast.remove(), 200);
			}, 1600);
		});
	}
"##;
