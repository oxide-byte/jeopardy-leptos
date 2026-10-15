pub mod features;
pub mod footer;
pub mod header;
pub mod hero;
pub mod rules_modal;

#[allow(unused_imports)]
pub use features::SplashFeatures;
#[allow(unused_imports)]
pub use footer::SplashFooter;
#[allow(unused_imports)]
pub use header::SplashHeader;
#[allow(unused_imports)]
pub use hero::SplashHero;
#[allow(unused_imports)]
pub use rules_modal::RulesModal;

use leptos::prelude::*;

#[component]
pub fn SplashScreen(
    #[prop(into)] on_start: Callback<()>,
) -> impl IntoView {
    let show_rules = RwSignal::new(false);

    let toggle_rules = move |_| {
        show_rules.update(|val| *val = !*val);
    };

    let close_rules = move |_| {
        show_rules.set(false);
    };

    let start_game = move |_| {
        on_start.run(());
    };

    view! {
        <div class="splash-container">
            <SplashHeader />

            <main class="splash-main">
                <SplashHero on_start=start_game on_toggle_rules=toggle_rules />
                <SplashFeatures />
            </main>

            <RulesModal show=show_rules on_close=close_rules />

            <SplashFooter />
        </div>
    }
}
