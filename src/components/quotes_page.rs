use crate::components::app::RefreshContext;
use crate::components::{CreateQuoteForm, QuotesList};
use crate::types::{QuoteWithTags, SearchParams};
use leptos::context::use_context;
use leptos::prelude::*;

#[component]
pub fn QuotesPage() -> impl IntoView {
    let (show_form, set_show_form) = signal(false);
    let (show_search, set_show_search) = signal(false);
    let (search_params, set_search_params) = signal(SearchParams::default());

    let refresh_context =
        use_context::<RefreshContext>().expect("RefreshContext should be provided");

    let on_quote_created = Callback::new(move |_new_quote: QuoteWithTags| {
        refresh_context.refresh_quotes.run(());
    });

    let on_search = move |_| {
        // Trigger search by updating the signal
        set_search_params.update(|params| {
            // Force signal update even if values are the same
            *params = params.clone();
        });
    };

    let clear_search = move |_| {
        set_search_params.set(SearchParams::default());
    };

    view! {
        <div class="quotes-page">
            <div class="page-header">
                <h2>"All Quotes"</h2>
                <div class="header-buttons">
                    <button
                        class="toggle-search-btn"
                        on:click=move |_| set_show_search.update(|show| *show = !*show)
                        tabindex="0"
                    >
                        {move || if show_search.get() { "✖ Hide Search" } else { "Search Quotes" }}
                    </button>
                    <button
                        class="toggle-form-btn"
                        on:click=move |_| set_show_form.update(|show| *show = !*show)
                        tabindex="0"
                    >
                        {move || if show_form.get() { "✖ Cancel" } else { "+ Add New Quote" }}
                    </button>
                </div>
            </div>

            <div class=move || if show_search.get() { "search-container visible" } else { "search-container hidden" }>
                <div class="search-form">
                    <h3>"Search Quotes"</h3>
                    <div class="search-fields">
                        <div class="field-group">
                            <label for="quote-search">"Quote Text:"</label>
                            <input
                                type="text"
                                id="quote-search"
                                placeholder="Search in quote text..."
                                prop:value=move || search_params.get().quote.unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_search_params.update(|params| {
                                        params.quote = if value.trim().is_empty() { None } else { Some(value) };
                                    });
                                }
                            />
                        </div>
                        <div class="field-group">
                            <label for="source-search">"Source/Author:"</label>
                            <input
                                type="text"
                                id="source-search"
                                placeholder="Search in source/author..."
                                prop:value=move || search_params.get().source.unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_search_params.update(|params| {
                                        params.source = if value.trim().is_empty() { None } else { Some(value) };
                                    });
                                }
                            />
                        </div>
                        <div class="field-group">
                            <label for="tag-search">"Tags:"</label>
                            <input
                                type="text"
                                id="tag-search"
                                placeholder="Search in tags..."
                                prop:value=move || search_params.get().tag.unwrap_or_default()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_search_params.update(|params| {
                                        params.tag = if value.trim().is_empty() { None } else { Some(value) };
                                    });
                                }
                            />
                        </div>
                    </div>
                    <div class="search-actions">
                        <button
                            class="search-btn"
                            on:click=on_search
                            tabindex="0"
                        >
                            "Search"
                        </button>
                        <button
                            class="clear-btn"
                            on:click=clear_search
                            tabindex="0"
                        >
                            "Clear"
                        </button>
                    </div>
                </div>
            </div>

            <div class=move || if show_form.get() { "form-container visible" } else { "form-container hidden" }>
                <CreateQuoteForm
                    on_success=Callback::new(move |_| set_show_form.set(false))
                    on_quote_created=on_quote_created
                />
            </div>

            <QuotesList
                refresh_trigger=refresh_context.refresh_counter
                search_params=search_params.into()
            />
        </div>
    }
}
