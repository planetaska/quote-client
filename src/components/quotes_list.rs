use crate::api::fetch_quotes;
use crate::components::QuoteCard;
use crate::types::SearchParams;
use leptos::prelude::*;

#[component]
pub fn QuotesList(
    #[prop(optional)] refresh_trigger: Option<Signal<u32>>,
    #[prop(optional)] search_params: Option<Signal<SearchParams>>,
) -> impl IntoView {
    let quotes_resource = LocalResource::new(move || {
        // Track the refresh trigger to re-run when it changes
        if let Some(trigger) = refresh_trigger {
            trigger.track();
        }
        // Track search params to re-run when they change
        let params = if let Some(search_signal) = search_params {
            search_signal.track();
            Some(search_signal.get())
        } else {
            None
        };
        fetch_quotes(params)
    });

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
                                                    <QuoteCard
                                                        quote=Signal::derive(move || quote.clone())
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
