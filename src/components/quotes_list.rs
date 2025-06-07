use crate::api::fetch_quotes;
use crate::components::QuoteCard;
use crate::types::QuoteWithTags;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn QuotesList() -> impl IntoView {
    let quotes_resource = LocalResource::new(|| fetch_quotes());
    let quotes_map = RwSignal::new(HashMap::<i64, QuoteWithTags>::new());

    let update_quote_in_list = Callback::new(move |updated_quote: QuoteWithTags| {
        quotes_map.update(|map| {
            map.insert(updated_quote.id, updated_quote);
        });
    });

    view! {
        <div class="quotes-container">
            <h1>"Quotes"</h1>
            
{move || Suspend::new(async move {
                match quotes_resource.await {
                    Ok(quotes) => {
                        // Initialize the quotes map
                        let mut initial_map = HashMap::new();
                        for quote in &quotes {
                            initial_map.insert(quote.id, quote.clone());
                        }
                        quotes_map.set(initial_map);

                        if quotes.is_empty() {
                            view! { <p>"No quotes available."</p> }.into_any()
                        } else {
                            view! {
                                <div class="quotes-grid">
                                    <For
                                        each=move || quotes.clone()
                                        key=|quote| quote.id
                                        children=move |quote| {
                                            let quote_id = quote.id;
                                            let quote_signal = Signal::derive(move || {
                                                quotes_map.with(|map| {
                                                    map.get(&quote_id).cloned().unwrap_or(quote.clone())
                                                })
                                            });
                                            
                                            view! {
                                                <QuoteCard 
                                                    quote=quote_signal
                                                    on_update=update_quote_in_list
                                                />
                                            }
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    }
                    Err(err) => {
                        view! { <p class="error">"Error: " {err}</p> }.into_any()
                    }
                }
            })}
        </div>
    }
}