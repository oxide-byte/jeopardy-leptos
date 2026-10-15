pub mod clue_card;
pub mod grid;
pub mod header;
pub mod modal;
pub mod types;

#[allow(unused_imports)]
pub use clue_card::ClueCard;
#[allow(unused_imports)]
pub use grid::{BoardGrid, CategoryHeader};
#[allow(unused_imports)]
pub use header::GameHeader;
#[allow(unused_imports)]
pub use modal::ClueModal;
pub use types::{default_categories, initial_clues, Clue, ClueStatus};

use leptos::prelude::*;

#[component]
pub fn GameBoard(
    #[prop(into)] on_exit: Callback<()>,
) -> impl IntoView {
    let score = RwSignal::new(0i32);
    let selected_clue = RwSignal::new(Option::<Clue>::None);
    let show_answer = RwSignal::new(false);

    let clues = RwSignal::new(initial_clues());
    let categories = default_categories();

    let open_clue = move |clue: Clue| {
        if clue.status == ClueStatus::Unanswered {
            selected_clue.set(Some(clue));
            show_answer.set(false);
        }
    };

    let close_modal = move |_| {
        selected_clue.set(None);
        show_answer.set(false);
    };

    let answer_correct = move |_| {
        if let Some(c) = selected_clue.get() {
            score.update(|s| *s += c.value as i32);
            clues.update(|list| {
                if let Some(target) = list.iter_mut().find(|item| item.id == c.id) {
                    target.status = ClueStatus::Correct;
                }
            });
            selected_clue.set(None);
            show_answer.set(false);
        }
    };

    let answer_incorrect = move |_| {
        if let Some(c) = selected_clue.get() {
            score.update(|s| *s -= c.value as i32);
            clues.update(|list| {
                if let Some(target) = list.iter_mut().find(|item| item.id == c.id) {
                    target.status = ClueStatus::Incorrect;
                }
            });
            selected_clue.set(None);
            show_answer.set(false);
        }
    };

    view! {
        <div class="game-container">
            <GameHeader on_exit=on_exit score=score />

            <BoardGrid
                categories=categories
                clues=clues
                on_select_clue=open_clue
            />

            <ClueModal
                clue=selected_clue
                show_answer=show_answer
                on_close=close_modal
                on_reveal=move |_| show_answer.set(true)
                on_correct=answer_correct
                on_incorrect=answer_incorrect
            />
        </div>
    }
}
