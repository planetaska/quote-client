use crate::components::{About, Home, Navigation, QuotesPage, RandomQuote, ShowQuote};
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

/// Context for triggering quote list refreshes across components.
#[derive(Clone, Copy)]
pub struct RefreshContext {
    pub refresh_quotes: Callback<()>,
    pub refresh_counter: Signal<u32>,
}

/// Root application component with routing and global context.
#[component]
pub fn App() -> impl IntoView {
    let (refresh_counter, set_refresh_counter) = signal(0u32);

    let refresh_quotes = Callback::new(move |_| {
        set_refresh_counter.update(|n| *n += 1);
    });

    let refresh_context = RefreshContext {
        refresh_quotes,
        refresh_counter: refresh_counter.into(),
    };

    view! {
        <div class="app">
            <a href="#main-content" class="skip-link">"Skip to main content"</a>
            <Provider value=refresh_context>
                <Router>
                    <header class="app-header" role="banner">
                        <Navigation />
                    </header>

                    <main id="main-content" class="main-content" role="main" tabindex="-1">
                        <Routes fallback=|| view! {
                            <div role="alert" aria-live="assertive">
                                <h1>"Page Not Found"</h1>
                                <p>"The requested page could not be found."</p>
                            </div>
                        }>
                            <Route path=StaticSegment("") view=Home />
                            <Route path=StaticSegment("random") view=RandomQuote />
                            <Route path=StaticSegment("quotes") view=QuotesPage />
                            <Route path=(StaticSegment("quote"), ParamSegment("id")) view=ShowQuote />
                            <Route path=StaticSegment("about") view=About />
                        </Routes>
                    </main>
                </Router>
            </Provider>
        </div>
    }
}
