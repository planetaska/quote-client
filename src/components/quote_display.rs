use crate::types::QuoteWithTags;
use leptos::prelude::*;

#[component]
pub fn QuoteDisplay(
    #[prop(into)] quote: Signal<QuoteWithTags>,
    #[prop(optional)] show_quote_marks: bool,
    #[prop(optional)] container_class: &'static str,
    #[prop(optional)] quote_class: &'static str,
    #[prop(optional)] source_class: &'static str,
    #[prop(optional)] tags_class: &'static str,
) -> impl IntoView {
    let container_class = if container_class.is_empty() {
        "quote-display"
    } else {
        container_class
    };
    let quote_class = if quote_class.is_empty() {
        "quote-text"
    } else {
        quote_class
    };
    let source_class = if source_class.is_empty() {
        "quote-source"
    } else {
        source_class
    };
    let tags_class = if tags_class.is_empty() {
        "quote-tags"
    } else {
        tags_class
    };

    view! {
        <div class=container_class>
            <blockquote class=quote_class>
                {move || {
                    let q = quote.get();
                    if show_quote_marks {
                        format!("\"{}\"", q.quote)
                    } else {
                        q.quote
                    }
                }}
            </blockquote>
            <p class=source_class>
                "— " {move || quote.get().source}
            </p>
            {move || {
                let q = quote.get();
                if !q.tags.is_empty() {
                    view! {
                        <div class=tags_class>
                            {q.tags.into_iter().map(|tag| {
                                view! {
                                    <span class="tag">{tag}</span>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
