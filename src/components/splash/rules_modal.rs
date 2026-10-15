use leptos::prelude::*;

#[component]
pub fn RulesModal(
    #[prop(into)] show: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        {move || {
            if show.get() {
                view! {
                    <div class="rules-modal-backdrop">
                        <div class="rules-modal-card">
                            <button
                                on:click=move |_| on_close.run(())
                                class="rules-modal-close"
                                aria-label="Close"
                            >
                                "✕"
                            </button>
                            <h3 class="rules-modal-title">
                                <span>"How To Play"</span>
                            </h3>
                            <div class="rules-list">
                                <div class="rules-item">
                                    <span class="rules-item-number">"1."</span>
                                    <p><strong class="rules-item-highlight">"Select a Clue:"</strong>" Choose a clue from any category on the board based on its dollar value."</p>
                                </div>
                                <div class="rules-item">
                                    <span class="rules-item-number">"2."</span>
                                    <p><strong class="rules-item-highlight">"Form of a Question:"</strong>" Every correct response must be phrased as a question (e.g., 'What is...', 'Who is...')."</p>
                                </div>
                                <div class="rules-item">
                                    <span class="rules-item-number">"3."</span>
                                    <p><strong class="rules-item-highlight">"Earn Cash:"</strong>" Correct answers add clue cash to your score, while incorrect responses deduct it."</p>
                                </div>
                                <div class="rules-item">
                                    <span class="rules-item-number">"4."</span>
                                    <p><strong class="rules-item-highlight">"Daily Double & Final:"</strong>" Wager wisely to maximize your total earnings!"</p>
                                </div>
                            </div>
                            <div class="rules-modal-footer">
                                <button
                                    on:click=move |_| on_close.run(())
                                    class="btn-got-it"
                                >
                                    "Got It!"
                                </button>
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
