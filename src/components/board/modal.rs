use super::types::Clue;
use leptos::prelude::*;

#[component]
pub fn ClueModal(
    #[prop(into)] clue: Signal<Option<Clue>>,
    #[prop(into)] show_answer: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_reveal: Callback<()>,
    #[prop(into)] on_correct: Callback<()>,
    #[prop(into)] on_incorrect: Callback<()>,
) -> impl IntoView {
    view! {
        {move || {
            if let Some(clue) = clue.get() {
                let clue_val = clue.value;
                let clue_cat = clue.category;
                let clue_q = clue.question;
                let clue_a = clue.answer;
                view! {
                    <div class="clue-modal-backdrop">
                        <div class="clue-modal-card">
                            <button
                                on:click=move |_| on_close.run(())
                                class="clue-modal-close"
                            >
                                "✕"
                            </button>
                            <div class="clue-modal-meta">
                                {format!("{} - ${}", clue_cat, clue_val)}
                            </div>
                            <div class="clue-modal-question">
                                {clue_q}
                            </div>

                            // Answer section
                            {move || {
                                if show_answer.get() {
                                    view! {
                                        <div class="clue-answer-card">
                                            <div class="clue-answer-label">"Correct Response"</div>
                                            <div class="clue-answer-text">{clue_a}</div>
                                        </div>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }
                            }}

                            <div class="clue-modal-actions">
                                {move || {
                                    if !show_answer.get() {
                                        view! {
                                            <button
                                                on:click=move |_| on_reveal.run(())
                                                class="btn-reveal-response"
                                            >
                                                "Reveal Response"
                                            </button>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="clue-response-buttons">
                                                <button
                                                    on:click=move |_| on_correct.run(())
                                                    class="btn-correct"
                                                >
                                                    <span>"✓ Correct"</span>
                                                </button>
                                                <button
                                                    on:click=move |_| on_incorrect.run(())
                                                    class="btn-incorrect"
                                                >
                                                    <span>"✕ Incorrect"</span>
                                                </button>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }
        }}
    }
}
