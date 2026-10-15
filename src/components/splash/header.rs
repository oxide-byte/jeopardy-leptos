use leptos::prelude::*;

#[component]
pub fn SplashHeader() -> impl IntoView {
    view! {
        <header class="splash-header">
            <div class="splash-header-group">
                <span class="splash-header-star">"★"</span>
                <span class="splash-header-subtitle">"America's Favorite Quiz Show"</span>
            </div>
            <div class="splash-badge">
                "Official Edition"
            </div>
        </header>
    }
}
