use crate::api::fetch_random_quote;
use crate::components::QuoteDisplay;
use crate::types::QuoteWithTags;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Home() -> impl IntoView {
    let (quote, set_quote) = signal::<Option<QuoteWithTags>>(None);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);

    let load_random_quote = move || {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match fetch_random_quote().await {
                Ok(q) => {
                    set_quote.set(Some(q));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    };

    // Load initial quote
    Effect::new(move |_| {
        load_random_quote();
    });

    view! {
        <div class="page-content">
            <div class="home-hero">
                <h1 class="hero-title">"Welcome to the Quotes Server!"</h1>
                <p class="hero-subtitle">"Serving up fresh inspiration 24/7 — no login, no nonsense, just quotes!"</p>
            </div>

            <div class="home-quote-section">

                {move || {
                    if loading.get() {
                        view! {
                            <div class="loading-spinner">
                                <div class="spinner"></div>
                                <p>"Loading quote..."</p>
                            </div>
                        }.into_any()
                    } else if let Some(error_msg) = error.get() {
                        view! {
                            <div class="error-message">
                                <p>"Error: " {error_msg}</p>
                                <button
                                    class="btn btn-secondary"
                                    on:click=move |_| load_random_quote()
                                    tabindex="0"
                                    aria-label="Retry loading a random quote"
                                >
                                    "Try Again"
                                </button>
                            </div>
                        }.into_any()
                    } else if let Some(q) = quote.get() {
                        let quote_signal = Signal::from(q);
                        view! {
                            <QuoteDisplay
                                quote=quote_signal
                                show_quote_marks=false
                                container_class="home-quote-display"
                                quote_class="home-quote-text"
                                source_class="home-quote-source"
                                tags_class="home-quote-tags"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="no-quote">
                                <p>"No quote available"</p>
                            </div>
                        }.into_any()
                    }
                }}

                <div class="home-quote-actions">
                    <button
                        class="btn btn-primary"
                        on:click=move |_| load_random_quote()
                        disabled=loading
                        tabindex="0"
                        aria-label="Get another random quote"
                    >
                        {move || if loading.get() { "Loading..." } else { "Get Another Quote" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
