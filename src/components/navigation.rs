use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="navigation">
            <ul class="nav-list">
                <li><A href="/" prop:class="nav-link">"Home"</A></li>
                <li><A href="/random" prop:class="nav-link">"Random Quote"</A></li>
                <li><A href="/quotes" prop:class="nav-link">"All Quotes"</A></li>
                <li><A href="/about" prop:class="nav-link">"About"</A></li>
            </ul>
        </nav>
    }
}