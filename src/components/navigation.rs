use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="navigation">
            <ul class="nav-list">
                <li><a href="/" class="nav-link">"Home"</a></li>
                <li><a href="/random" class="nav-link">"Random Quote"</a></li>
                <li><a href="/quotes" class="nav-link">"All Quotes"</a></li>
                <li><a href="/about" class="nav-link">"About"</a></li>
            </ul>
        </nav>
    }
}