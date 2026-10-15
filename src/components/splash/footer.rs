use leptos::prelude::*;

#[component]
pub fn SplashFooter() -> impl IntoView {
    view! {
        <footer class="splash-footer">
            <span>"Jeopardy! Quiz Application"</span>
            <span class="splash-footer-highlight">"Built with Leptos & WebAssembly"</span>
        </footer>
    }
}
