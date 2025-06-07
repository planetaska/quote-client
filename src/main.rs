use leptos::prelude::*;

mod api;
mod components;
mod types;

use components::QuotesList;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <QuotesList /> })
}