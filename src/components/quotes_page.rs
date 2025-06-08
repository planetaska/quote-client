use crate::components::app::RefreshContext;
use crate::components::{CreateQuoteForm, QuotesList};
use crate::types::QuoteWithTags;
use leptos::context::use_context;
use leptos::prelude::*;

#[component]
pub fn QuotesPage() -> impl IntoView {
    let (show_form, set_show_form) = signal(false);
    let refresh_context =
        use_context::<RefreshContext>().expect("RefreshContext should be provided");

    let on_quote_created = Callback::new(move |_new_quote: QuoteWithTags| {
        refresh_context.refresh_quotes.run(());
    });

    view! {
        <div class="quotes-page">
            <div class="page-header">
                <h2>"All Quotes"</h2>
                <button
                    class="toggle-form-btn"
                    on:click=move |_| set_show_form.update(|show| *show = !*show)
                    tabindex="0"
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

            <QuotesList refresh_trigger=refresh_context.refresh_counter />
        </div>
    }
}
