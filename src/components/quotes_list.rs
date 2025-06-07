use crate::api::fetch_quotes;
use crate::components::QuoteCard;
use leptos::prelude::*;

#[component]
pub fn QuotesList() -> impl IntoView {
    let quotes_resource = LocalResource::new(|| fetch_quotes());

    view! {
        <div class="quotes-container">
            <h1>"Quotes"</h1>
            
{move || Suspend::new(async move {
                match quotes_resource.await {
                    Ok(quotes) => {
                        if quotes.is_empty() {
                            view! { <p>"No quotes available."</p> }.into_any()
                        } else {
                            view! {
                                <div class="quotes-grid">
                                    <For
                                        each=move || quotes.clone()
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
                    Err(err) => {
                        view! { <p class="error">"Error: " {err}</p> }.into_any()
                    }
                }
            })}
        </div>
    }
}