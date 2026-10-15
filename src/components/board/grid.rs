use super::clue_card::ClueCard;
use super::types::Clue;
use leptos::prelude::*;

#[component]
pub fn CategoryHeader(category: &'static str) -> impl IntoView {
    view! {
        <div class="category-header">
            {category}
        </div>
    }
}

#[component]
pub fn BoardGrid(
    categories: Vec<&'static str>,
    #[prop(into)] clues: Signal<Vec<Clue>>,
    #[prop(into)] on_select_clue: Callback<Clue>,
) -> impl IntoView {
    view! {
        <main class="game-main">
            <div class="game-board-grid">
                // Category Headers
                {categories.into_iter().map(|cat| {
                    view! {
                        <CategoryHeader category=cat />
                    }
                }).collect_view()}

                // Clue Values (by row, 200 to 1000)
                {(0..5).map(|row| {
                    let value = ((row + 1) * 200) as u32;
                    (0..6).map(move |col| {
                        let clue_idx = col * 5 + row;
                        let clue_signal = Memo::new(move |_| {
                            clues.get().get(clue_idx).cloned()
                        });

                        view! {
                            <ClueCard
                                clue=clue_signal
                                value=value
                                on_select=on_select_clue
                            />
                        }
                    }).collect_view()
                }).collect_view()}
            </div>
        </main>
    }
}
