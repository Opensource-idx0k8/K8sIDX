#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Heading(cx: Scope) -> Element {
	render!(
		div {
			h3 { class: "w-full h-screen bg-gray-300 flex items-center justify-center", "Hello, World!" }
		}
	)
}