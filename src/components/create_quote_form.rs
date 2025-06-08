use crate::api::create_quote;
use crate::types::{CreateQuoteRequest, QuoteWithTags};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn CreateQuoteForm(
    on_success: Callback<()>,
    #[prop(optional)] on_quote_created: Option<Callback<QuoteWithTags>>,
) -> impl IntoView {
    let (quote_text, set_quote_text) = signal(String::new());
    let (source, set_source) = signal(String::new());
    let (tags_input, set_tags_input) = signal(String::new());
    let (creating, set_creating) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    let (success, set_success) = signal(false);

    let submit_quote = move |_| {
        let quote = quote_text.get_untracked();
        let source_val = source.get_untracked();
        let tags_str = tags_input.get_untracked();

        if quote.trim().is_empty() || source_val.trim().is_empty() {
            set_error.set(Some("Quote and source are required".to_string()));
            return;
        }

        let tags = if tags_str.trim().is_empty() {
            None
        } else {
            Some(tags_str.split(',').map(|s| s.trim().to_string()).collect())
        };

        let request = CreateQuoteRequest {
            quote,
            source: source_val,
            tags,
        };

        spawn_local(async move {
            set_creating.set(true);
            set_error.set(None);
            set_success.set(false);

            match create_quote(request).await {
                Ok(new_quote) => {
                    set_quote_text.set(String::new());
                    set_source.set(String::new());
                    set_tags_input.set(String::new());
                    set_success.set(true);
                    set_creating.set(false);

                    // Notify about the new quote immediately
                    if let Some(on_quote_created) = on_quote_created {
                        on_quote_created.run(new_quote);
                    }

                    // Call the success callback after a short delay to show success message
                    set_timeout(
                        move || on_success.run(()),
                        std::time::Duration::from_millis(1500),
                    );
                }
                Err(err) => {
                    set_error.set(Some(err));
                    set_creating.set(false);
                }
            }
        });
    };

    view! {
        <div class="create-quote-form">
            <h2 id="form-title">"Add New Quote"</h2>

            {move || {
                if let Some(err) = error.get() {
                    view! { <p class="error" role="alert" aria-live="polite">"Error: " {err}</p> }.into_any()
                } else if success.get() {
                    view! { <p class="success" role="status" aria-live="polite">"Quote created successfully!"</p> }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            <form
                on:submit=move |e| {
                    e.prevent_default();
                    submit_quote(e);
                }
                aria-labelledby="form-title"
                novalidate
            >
                <div class="form-group">
                    <label for="quote">"Quote" <span aria-hidden="true">"*"</span></label>
                    <textarea
                        id="quote"
                        prop:value=move || quote_text.get()
                        on:input=move |e| set_quote_text.set(event_target_value(&e))
                        placeholder="Enter the quote text..."
                        required
                        aria-required="true"
                        aria-describedby="quote-desc"
                    ></textarea>
                    <div id="quote-desc" class="sr-only">"Required field. Enter the main text of the quote."</div>
                </div>

                <div class="form-group">
                    <label for="source">"Source" <span aria-hidden="true">"*"</span></label>
                    <input
                        type="text"
                        id="source"
                        prop:value=move || source.get()
                        on:input=move |e| set_source.set(event_target_value(&e))
                        placeholder="Enter the source (author, book, etc.)..."
                        required
                        aria-required="true"
                        aria-describedby="source-desc"
                    />
                    <div id="source-desc" class="sr-only">"Required field. Enter the author, book, or source of the quote."</div>
                </div>

                <div class="form-group">
                    <label for="tags">"Tags (comma-separated)"</label>
                    <input
                        type="text"
                        id="tags"
                        prop:value=move || tags_input.get()
                        on:input=move |e| set_tags_input.set(event_target_value(&e))
                        placeholder="philosophy, wisdom, inspiration..."
                        aria-describedby="tags-desc"
                    />
                    <div id="tags-desc" class="sr-only">"Optional field. Enter comma-separated tags to categorize the quote."</div>
                </div>

                <button
                    type="submit"
                    disabled=move || creating.get()
                    aria-describedby="submit-desc"
                    tabindex="0"
                >
                    {move || if creating.get() { "Creating..." } else { "Create Quote" }}
                </button>
                <div id="submit-desc" class="sr-only">"Submit the form to create a new quote. Quote and source fields are required."</div>
            </form>
        </div>
    }
}
