use crate::components::{GameBoard, SplashScreen};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let in_game = RwSignal::new(false);

    let start_game = move |_| {
        in_game.set(true);
    };

    let exit_game = move |_| {
        in_game.set(false);
    };

    view! {
        <main class="app-main">
            {move || {
                if in_game.get() {
                    view! {
                        <GameBoard on_exit=exit_game />
                    }.into_any()
                } else {
                    view! {
                        <SplashScreen on_start=start_game />
                    }.into_any()
                }
            }}
        </main>
    }
}
