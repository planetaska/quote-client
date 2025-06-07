use crate::api::fetch_quotes;
use crate::types::QuoteWithTags;
use leptos::prelude::*;

#[component]
pub fn QuoteCard(quote: QuoteWithTags) -> impl IntoView {
    view! {
        <div class="quote-card">
            <blockquote class="quote-text">
                {quote.quote}
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