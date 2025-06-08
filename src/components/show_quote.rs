use crate::api::fetch_quote_by_id;
use crate::components::QuoteDisplay;
use crate::types::QuoteWithTags;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ShowQuote() -> impl IntoView {
    let params = use_params_map();
    let (quote, set_quote) = signal::<Option<QuoteWithTags>>(None);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);

    let load_quote = move || {
        let id_str = params
            .get()
            .get("id")
            .unwrap_or_default();

        if let Ok(id) = id_str.parse::<u32>() {
            set_loading.set(true);
            set_error.set(None);

            spawn_local(async move {
                match fetch_quote_by_id(id).await {
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
        } else {
            set_error.set(Some("Invalid quote ID".to_string()));
        }
    };

    // Load quote when component mounts or when params change
    Effect::new(move |_| {
        load_quote();
    });

    view! {
        <div class="page-content">
            <div class="show-quote-container">
                <h1 class="page-title">"Quote"</h1>

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
                                    on:click=move |_| load_quote()
                                    tabindex="0"
                                    aria-label="Retry loading the quote"
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
                            <div class="text-center mt-16">
                                <A href="/quotes" attr:class="btn btn-primary" attr:aria-label="Navigate to all quotes page" attr:tabindex="0">Show all quotes</A>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="no-quote">
                                <p>"No quote found"</p>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
