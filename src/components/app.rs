use crate::components::{Navigation, Home, RandomQuote, About, QuotesPage, ShowQuote};
use leptos::prelude::*;
use leptos::context::Provider;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::{StaticSegment, ParamSegment};

// Global refresh context
#[derive(Clone, Copy)]
pub struct RefreshContext {
    pub refresh_quotes: Callback<()>,
}

#[component]
pub fn App() -> impl IntoView {
    let (_refresh_counter, set_refresh_counter) = signal(0u32);

    let refresh_quotes = Callback::new(move |_| {
        set_refresh_counter.update(|n| *n += 1);
    });

    let refresh_context = RefreshContext { refresh_quotes };

    view! {
        <div class="app">
            <Provider value=refresh_context>
                <Router>
                    <div class="app-header">
                        <Navigation />
                    </div>
                    
                    <main class="main-content">
                        <Routes fallback=|| "Page not found">
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