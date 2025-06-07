use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::api::fetch_random_quote;
use crate::types::QuoteWithTags;
use crate::components::QuoteDisplay;

#[component]
pub fn RandomQuote() -> impl IntoView {
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
            <div class="random-quote-container">
                <h1 class="page-title">"Random Quote"</h1>
                
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
                
                <div class="quote-actions">
                    <button 
                        class="btn btn-primary next-quote-btn"
                        on:click=move |_| load_random_quote()
                        disabled=loading
                    >
                        {move || if loading.get() { "Loading..." } else { "Next Quote" }}
                    </button>
                </div>
            </div>
        </div>
    }
}