use crate::api::{delete_quote, update_quote};
use crate::components::app::RefreshContext;
use crate::types::{QuoteWithTags, UpdateQuoteRequest};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

#[component]
pub fn QuoteCard(#[prop(into)] quote: Signal<QuoteWithTags>) -> impl IntoView {
    let is_editing = RwSignal::new(false);
    let edit_quote = RwSignal::new(String::new());
    let edit_source = RwSignal::new(String::new());
    let edit_tags = RwSignal::new(String::new());
    let is_saving = RwSignal::new(false);
    let is_deleting = RwSignal::new(false);

    let refresh_context = expect_context::<RefreshContext>();
    let navigate = use_navigate();

    let start_edit = move |_| {
        let current_quote = quote.get();
        edit_quote.set(current_quote.quote);
        edit_source.set(current_quote.source);
        edit_tags.set(current_quote.tags.join(", "));
        is_editing.set(true);
    };

    let cancel_edit = move |_| {
        is_editing.set(false);
    };

    let save_edit = move |_| {
        let current_quote = quote.get();
        let tags_vec: Vec<String> = edit_tags
            .get()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let update_request = UpdateQuoteRequest {
            quote: edit_quote.get(),
            source: edit_source.get(),
            tags: if tags_vec.is_empty() {
                None
            } else {
                Some(tags_vec)
            },
        };

        is_saving.set(true);
        spawn_local(async move {
            match update_quote(current_quote.id, update_request).await {
                Ok(_updated_quote) => {
                    is_editing.set(false);
                    // Trigger refresh of the quotes list
                    refresh_context.refresh_quotes.run(());
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to update quote: {}", e).into());
                }
            }
            is_saving.set(false);
        });
    };

    let delete_quote_action = move |_| {
        let current_quote = quote.get();

        if web_sys::window()
            .unwrap()
            .confirm_with_message(&format!(
                "Are you sure you want to delete this quote?\n\n\"{}\"",
                current_quote.quote
            ))
            .unwrap_or(false)
        {
            is_deleting.set(true);
            spawn_local(async move {
                match delete_quote(current_quote.id).await {
                    Ok(_) => {
                        refresh_context.refresh_quotes.run(());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to delete quote: {}", e).into());
                    }
                }
                is_deleting.set(false);
            });
        }
    };

    view! {
        <div class="quote-card">
            {move || {
                let current_quote = quote.get();
                if is_editing.get() {
                    view! {
                        <div class="edit-form">
                            <textarea
                                class="edit-quote-input"
                                placeholder="Quote text"
                                prop:value=edit_quote
                                on:input=move |ev| edit_quote.set(event_target_value(&ev))
                            />
                            <input
                                type="text"
                                class="edit-source-input"
                                placeholder="Source"
                                prop:value=edit_source
                                on:input=move |ev| edit_source.set(event_target_value(&ev))
                            />
                            <input
                                type="text"
                                class="edit-tags-input"
                                placeholder="Tags (comma-separated)"
                                prop:value=edit_tags
                                on:input=move |ev| edit_tags.set(event_target_value(&ev))
                            />
                            <div class="edit-buttons">
                                <button
                                    class="save-button"
                                    on:click=save_edit
                                    disabled=is_saving
                                    tabindex="0"
                                >
                                    {move || if is_saving.get() { "Saving..." } else { "Save" }}
                                </button>
                                <button
                                    class="cancel-button"
                                    on:click=cancel_edit
                                    disabled=is_saving
                                    tabindex="0"
                                >
                                    "Cancel"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <blockquote class="quote-text">
                            {current_quote.quote}
                        </blockquote>
                        <cite class="quote-source">
                            "— " {current_quote.source}
                        </cite>
                        {(!current_quote.tags.is_empty()).then(|| view! {
                            <div class="quote-tags">
                                <For
                                    each=move || current_quote.tags.clone()
                                    key=|tag| tag.clone()
                                    children=move |tag| {
                                        view! {
                                            <span class="tag">{tag}</span>
                                        }
                                    }
                                />
                            </div>
                        })}
                        <div class="quote-actions">
                            <button
                                class="edit-button"
                                on:click={
                                    let navigate = navigate.clone();
                                    move |_| {
                                        let id = current_quote.id;
                                        navigate(&format!("/quote/{}", id), Default::default());
                                    }
                                }
                                tabindex="0"
                            >
                                "Show"
                            </button>
                            <button class="edit-button" on:click=start_edit tabindex="0">
                                "Edit"
                            </button>
                            <button
                                class="delete-button bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded ml-2 disabled:opacity-50"
                                on:click=delete_quote_action
                                disabled=is_deleting
                                tabindex="0"
                            >
                                {move || if is_deleting.get() { "Deleting..." } else { "Delete" }}
                            </button>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
