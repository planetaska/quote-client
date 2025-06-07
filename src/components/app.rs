use crate::components::{CreateQuoteForm, QuotesList};
use crate::types::QuoteWithTags;
use leptos::prelude::*;
use leptos::context::Provider;

// Global refresh context
#[derive(Clone, Copy)]
pub struct RefreshContext {
    pub refresh_quotes: Callback<()>,
}

#[component]
pub fn App() -> impl IntoView {
    let (show_form, set_show_form) = signal(false);
    let (refresh_counter, set_refresh_counter) = signal(0u32);

    let refresh_quotes = Callback::new(move |_| {
        set_refresh_counter.update(|n| *n += 1);
    });

    let refresh_context = RefreshContext { refresh_quotes };

    let on_quote_created = Callback::new(move |_new_quote: QuoteWithTags| {
        // Trigger refresh by incrementing counter
        refresh_quotes.run(());
    });

    view! {
        <div class="app">
            <Provider value=refresh_context>
                <div class="app-header">
                    <h1>"Quote Server"</h1>
                    <button 
                        class="toggle-form-btn"
                        on:click=move |_| set_show_form.update(|show| *show = !*show)
                    >
                        {move || if show_form.get() { "✖ Cancel" } else { "+ Add New Quote" }}
                    </button>
                </div>
                
                <div class=move || if show_form.get() { "form-container visible" } else { "form-container hidden" }>
                    <CreateQuoteForm 
                        on_success=Callback::new(move |_| set_show_form.set(false))
                        on_quote_created=on_quote_created
                    />
                </div>
                
                <QuotesList refresh_trigger=refresh_counter.into() />
            </Provider>
        </div>
    }
}