use leptos::prelude::*;

#[component]
pub fn SplashHero(
    #[prop(into)] on_start: Callback<()>,
    #[prop(into)] on_toggle_rules: Callback<()>,
) -> impl IntoView {
    view! {
        // Jeopardy Logo Badge
        <div class="splash-logo-container">
            <div class="splash-logo-glow"></div>
            <div class="splash-logo-box">
                <h1 class="splash-logo-title">
                    "JEOPARDY!"
                </h1>
            </div>
        </div>

        // Welcome Subtitle
        <div class="splash-text-container">
            <h2 class="splash-title">
                "Welcome to Jeopardy!"
            </h2>
            <p class="splash-description">
                "Put your knowledge to the ultimate test. Select your categories, buzz in with confidence, and remember: always answer in the form of a question!"
            </p>
        </div>

        // Action Buttons
        <div class="splash-actions">
            <button
                on:click=move |_| on_start.run(())
                class="btn-play"
            >
                <svg class="btn-play-icon" viewBox="0 0 24 24">
                    <path d="M8 5v14l11-7z"/>
                </svg>
                <span>"Play Now"</span>
            </button>

            <button
                on:click=move |_| on_toggle_rules.run(())
                class="btn-how-to-play"
            >
                <svg class="btn-how-to-play-icon" viewBox="0 0 24 24">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/>
                </svg>
                <span>"How to Play"</span>
            </button>
        </div>
    }
}
