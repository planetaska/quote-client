use crate::api::fetch_quotes;
use crate::types::QuoteWithTags;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn QuotesList() -> impl IntoView {
    let (quotes, set_quotes) = signal::<Vec<QuoteWithTags>>(vec![]);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal::<Option<String>>(None);

    // Load quotes on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match fetch_quotes().await {
                Ok(quotes_data) => {
                    set_quotes.set(quotes_data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="quotes-container">
            <h1>"Quotes"</h1>
            
            {move || {
                if loading.get() {
                    view! { <p>"Loading quotes..."</p> }.into_any()
                } else if let Some(err) = error.get() {
                    view! { <p class="error">"Error: " {err}</p> }.into_any()
                } else {
                    let quotes_list = quotes.get();
                    if quotes_list.is_empty() {
                        view! { <p>"No quotes available."</p> }.into_any()
                    } else {
                        view! {
                            <div class="quotes-grid">
                                <For
                                    each=move || quotes.get()
                                    key=|quote| quote.id
                                    children=move |quote| {
                                        view! {
                                            <QuoteCard quote=quote />
                                        }
                                    }
                                />
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}

#[component]
fn QuoteCard(quote: QuoteWithTags) -> impl IntoView {
    view! {
        <div class="quote-card">
            <blockquote class="quote-text">
                "\"" {quote.quote} "\""
            </blockquote>
            <cite class="quote-source">
                "— " {quote.source}
            </cite>
            {(!quote.tags.is_empty()).then(|| view! {
                <div class="quote-tags">
                    <For
                        each=move || quote.tags.clone()
                        key=|tag| tag.clone()
                        children=move |tag| {
                            view! {
                                <span class="tag">{tag}</span>
                            }
                        }
                    />
                </div>
            })}
        </div>
    }
}