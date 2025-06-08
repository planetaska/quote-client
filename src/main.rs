//! Quote client application built with Leptos and Trunk.
//! Consumes a REST API server running on localhost:3000.

use leptos::prelude::*;

mod api;
mod components;
mod types;

use components::App;

/// Entry point for the web application.
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}
