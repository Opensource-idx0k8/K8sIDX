#![allow(non_snake_case)]

mod heading;

use dioxus::prelude::*;

use heading::Heading;

fn main() {
	// Dioxus renders app in Webview
	dioxus_desktop::launch(App);
}

// Define a component that renders a div
fn App(cx: Scope) -> Element {
	// The render! macro makes it easy for developers to write jsx-style markup in their components.
	render! {
		link { rel: "stylesheet", href: "../dist/output.css" },
		Heading {}
	}
}