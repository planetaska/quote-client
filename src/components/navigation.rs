use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="navigation" role="navigation" aria-label="Main navigation">
            <ul class="nav-list" role="list">
                <li role="listitem">
                    <A href="/" attr:class="nav-link" attr:aria-label="Go to home page" attr:tabindex="0">"Home"</A>
                </li>
                <li role="listitem">
                    <A href="/random" attr:class="nav-link" attr:aria-label="View a random quote" attr:tabindex="0">
                    "Random"
                    <span class="hidden sm:inline">" Quote"</span>
                    </A>
                </li>
                <li role="listitem">
                    <A href="/quotes" attr:class="nav-link" attr:aria-label="Browse all quotes" attr:tabindex="0">
                    "All"
                    <span class="hidden sm:inline">" Quotes"</span>
                    </A>
                </li>
                <li role="listitem">
                    <A href="/about" attr:class="nav-link" attr:aria-label="Learn about this application" attr:tabindex="0">"About"</A>
                </li>
            </ul>
        </nav>
    }
}
