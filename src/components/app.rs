use crate::components::{CreateQuoteForm, QuotesList};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (show_form, set_show_form) = signal(false);

    view! {
        <div class="app">
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
                <CreateQuoteForm on_success=Callback::new(move |_| set_show_form.set(false)) />
            </div>
            
            <QuotesList />
        </div>
    }
}