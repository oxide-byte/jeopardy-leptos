use leptos::prelude::*;

#[component]
pub fn SplashFeatures() -> impl IntoView {
    view! {
        <div class="splash-features">
            <div class="splash-feature-card">
                <div class="splash-feature-icon-wrapper">
                    <svg class="splash-feature-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                    </svg>
                </div>
                <div>
                    <h3 class="splash-feature-title">"Classic Board"</h3>
                    <p class="splash-feature-desc">"Multiple categories with increasing dollar values from $200 to $1000."</p>
                </div>
            </div>

            <div class="splash-feature-card">
                <div class="splash-feature-icon-wrapper">
                    <svg class="splash-feature-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                    </svg>
                </div>
                <div>
                    <h3 class="splash-feature-title">"Daily Doubles"</h3>
                    <p class="splash-feature-desc">"Wager your earnings for high-stakes trivia rewards."</p>
                </div>
            </div>

            <div class="splash-feature-card">
                <div class="splash-feature-icon-wrapper">
                    <svg class="splash-feature-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>
                    </svg>
                </div>
                <div>
                    <h3 class="splash-feature-title">"Final Jeopardy"</h3>
                    <p class="splash-feature-desc">"Test your nerve in the ultimate championship question."</p>
                </div>
            </div>
        </div>
    }
}
