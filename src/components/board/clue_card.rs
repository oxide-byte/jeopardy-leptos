use super::types::{Clue, ClueStatus};
use leptos::prelude::*;

#[component]
pub fn ClueCard(
    #[prop(into)] clue: Signal<Option<Clue>>,
    value: u32,
    #[prop(into)] on_select: Callback<Clue>,
) -> impl IntoView {
    view! {
        {move || {
            if let Some(c) = clue.get() {
                let c_clone = c.clone();
                match c.status {
                    ClueStatus::Correct => view! {
                        <div class="clue-card-correct">
                            <span class="clue-status-correct">"O"</span>
                        </div>
                    }.into_any(),
                    ClueStatus::Incorrect => view! {
                        <div class="clue-card-incorrect">
                            <span class="clue-status-incorrect">"X"</span>
                        </div>
                    }.into_any(),
                    ClueStatus::Unanswered => view! {
                        <button
                            on:click=move |_| on_select.run(c_clone.clone())
                            class="clue-button"
                        >
                            {format!("${}", value)}
                        </button>
                    }.into_any(),
                }
            } else {
                view! { <div class="clue-placeholder"></div> }.into_any()
            }
        }}
    }
}
