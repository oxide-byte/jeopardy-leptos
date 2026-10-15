use leptos::prelude::*;

#[component]
pub fn GameHeader(
    #[prop(into)] on_exit: Callback<()>,
    #[prop(into)] score: Signal<i32>,
) -> impl IntoView {
    view! {
        <header class="game-header">
            <button
                on:click=move |_| on_exit.run(())
                class="btn-back"
            >
                <span>"← Back to Welcome"</span>
            </button>

            <div class="game-logo">
                "JEOPARDY!"
            </div>

            <div class="game-score-box">
                <span class="game-score-label">"Score:"</span>
                <span class="game-score-value">
                    {move || {
                        let s = score.get();
                        if s < 0 {
                            format!("-${}", s.abs())
                        } else {
                            format!("${}", s)
                        }
                    }}
                </span>
            </div>
        </header>
    }
}
