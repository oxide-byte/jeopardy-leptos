#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClueStatus {
    Unanswered,
    Correct,
    Incorrect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Clue {
    pub id: usize,
    pub category: &'static str,
    pub value: u32,
    pub question: &'static str,
    pub answer: &'static str,
    pub status: ClueStatus,
}

pub fn initial_clues() -> Vec<Clue> {
    vec![
        // Category 1: WORLD HISTORY
        Clue { id: 1, category: "WORLD HISTORY", value: 200, question: "This French emperor was exiled to the island of Elba in 1814.", answer: "Who is Napoleon Bonaparte?", status: ClueStatus::Unanswered },
        Clue { id: 2, category: "WORLD HISTORY", value: 400, question: "This ancient trade network connected China to the Mediterranean world.", answer: "What is the Silk Road?", status: ClueStatus::Unanswered },
        Clue { id: 3, category: "WORLD HISTORY", value: 600, question: "In 1215, King John of England was forced to sign this landmark charter.", answer: "What is Magna Carta?", status: ClueStatus::Unanswered },
        Clue { id: 4, category: "WORLD HISTORY", value: 800, question: "This Mesoamerican civilization built their capital city at Tenochtitlan.", answer: "Who are the Aztecs?", status: ClueStatus::Unanswered },
        Clue { id: 5, category: "WORLD HISTORY", value: 1000, question: "This 1648 series of peace treaties ended the Thirty Years' War in Europe.", answer: "What is the Peace of Westphalia?", status: ClueStatus::Unanswered },

        // Category 2: SCIENCE & NATURE
        Clue { id: 6, category: "SCIENCE & NATURE", value: 200, question: "This planet is known as the Red Planet due to iron oxide on its surface.", answer: "What is Mars?", status: ClueStatus::Unanswered },
        Clue { id: 7, category: "SCIENCE & NATURE", value: 400, question: "This gas makes up approximately 78 percent of Earth's atmosphere.", answer: "What is Nitrogen?", status: ClueStatus::Unanswered },
        Clue { id: 8, category: "SCIENCE & NATURE", value: 600, question: "This powerhouse organelle produces most of a cell's supply of ATP.", answer: "What is Mitochondria?", status: ClueStatus::Unanswered },
        Clue { id: 9, category: "SCIENCE & NATURE", value: 800, question: "In physics, this constant (approx. 3.00 × 10^8 m/s) is denoted by the letter c.", answer: "What is the speed of light?", status: ClueStatus::Unanswered },
        Clue { id: 10, category: "SCIENCE & NATURE", value: 1000, question: "This fundamental element with atomic number 1 is the most abundant in the universe.", answer: "What is Hydrogen?", status: ClueStatus::Unanswered },

        // Category 3: LITERATURE
        Clue { id: 11, category: "LITERATURE", value: 200, question: "This Shakespearean tragedy features the Prince of Denmark seeking revenge.", answer: "What is Hamlet?", status: ClueStatus::Unanswered },
        Clue { id: 12, category: "LITERATURE", value: 400, question: "George Orwell's dystopian 1949 novel introduced the concept of Big Brother.", answer: "What is 1984?", status: ClueStatus::Unanswered },
        Clue { id: 13, category: "LITERATURE", value: 600, question: "This Herman Melville classic begins with the famous line 'Call me Ishmael'.", answer: "What is Moby-Dick?", status: ClueStatus::Unanswered },
        Clue { id: 14, category: "LITERATURE", value: 800, question: "In this epic poem by Homer, Odysseus undertakes a ten-year journey home to Ithaca.", answer: "What is The Odyssey?", status: ClueStatus::Unanswered },
        Clue { id: 15, category: "LITERATURE", value: 1000, question: "This Russian author wrote 'War and Peace' and 'Anna Karenina'.", answer: "Who is Leo Tolstoy?", status: ClueStatus::Unanswered },

        // Category 4: GEOGRAPHY
        Clue { id: 16, category: "GEOGRAPHY", value: 200, question: "This is the largest ocean on Earth by surface area.", answer: "What is the Pacific Ocean?", status: ClueStatus::Unanswered },
        Clue { id: 17, category: "GEOGRAPHY", value: 400, question: "This is the longest river in the world, running through northeastern Africa.", answer: "What is the Nile?", status: ClueStatus::Unanswered },
        Clue { id: 18, category: "GEOGRAPHY", value: 600, question: "This European capital city is famously divided by the Danube River into Buda and Pest.", answer: "What is Budapest?", status: ClueStatus::Unanswered },
        Clue { id: 19, category: "GEOGRAPHY", value: 800, question: "This is the highest mountain peak in North America, located in Alaska.", answer: "What is Denali (Mount McKinley)?", status: ClueStatus::Unanswered },
        Clue { id: 20, category: "GEOGRAPHY", value: 1000, question: "This landlocked country in South America has two capital cities: La Paz and Sucre.", answer: "What is Bolivia?", status: ClueStatus::Unanswered },

        // Category 5: POP CULTURE
        Clue { id: 21, category: "POP CULTURE", value: 200, question: "This British rock band was composed of John, Paul, George, and Ringo.", answer: "Who are The Beatles?", status: ClueStatus::Unanswered },
        Clue { id: 22, category: "POP CULTURE", value: 400, question: "In the Marvel Cinematic Universe, this actor portrays Tony Stark / Iron Man.", answer: "Who is Robert Downey Jr.?", status: ClueStatus::Unanswered },
        Clue { id: 23, category: "POP CULTURE", value: 600, question: "This video game franchise created by Nintendo features Link rescuing Princess Zelda.", answer: "What is The Legend of Zelda?", status: ClueStatus::Unanswered },
        Clue { id: 24, category: "POP CULTURE", value: 800, question: "This artist released the best-selling album of all time, 'Thriller', in 1982.", answer: "Who is Michael Jackson?", status: ClueStatus::Unanswered },
        Clue { id: 25, category: "POP CULTURE", value: 1000, question: "This 1994 film won six Academy Awards, including Best Picture, starring Tom Hanks.", answer: "What is Forrest Gump?", status: ClueStatus::Unanswered },

        // Category 6: POTPOURRI
        Clue { id: 26, category: "POTPOURRI", value: 200, question: "This piece in chess can move any number of squares diagonally, horizontally, or vertically.", answer: "What is the Queen?", status: ClueStatus::Unanswered },
        Clue { id: 27, category: "POTPOURRI", value: 400, question: "This is the only mammal capable of sustained true flight.", answer: "What is the Bat?", status: ClueStatus::Unanswered },
        Clue { id: 28, category: "POTPOURRI", value: 600, question: "In Greek mythology, this hero had only one vulnerable spot on his heel.", answer: "Who is Achilles?", status: ClueStatus::Unanswered },
        Clue { id: 29, category: "POTPOURRI", value: 800, question: "This Japanese art form involves folding paper into intricate decorative shapes.", answer: "What is Origami?", status: ClueStatus::Unanswered },
        Clue { id: 30, category: "POTPOURRI", value: 1000, question: "This traditional Italian dessert literally translates to 'pick me up' or 'cheer me up'.", answer: "What is Tiramisu?", status: ClueStatus::Unanswered },
    ]
}

pub fn default_categories() -> Vec<&'static str> {
    vec![
        "WORLD HISTORY",
        "SCIENCE & NATURE",
        "LITERATURE",
        "GEOGRAPHY",
        "POP CULTURE",
        "POTPOURRI",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_clues_count_and_uniqueness() {
        let clues = initial_clues();
        assert_eq!(clues.len(), 30);

        let mut ids = std::collections::HashSet::new();
        for clue in &clues {
            assert!(ids.insert(clue.id), "Duplicate clue ID found: {}", clue.id);
            assert_eq!(clue.status, ClueStatus::Unanswered);
            assert!(!clue.question.is_empty());
            assert!(!clue.answer.is_empty());
        }
    }

    #[test]
    fn test_categories_and_values() {
        let categories = default_categories();
        let clues = initial_clues();

        assert_eq!(categories.len(), 6);

        let expected_values = [200, 400, 600, 800, 1000];

        for (cat_idx, &category) in categories.iter().enumerate() {
            let cat_clues: Vec<&Clue> = clues.iter().filter(|c| c.category == category).collect();
            assert_eq!(
                cat_clues.len(),
                5,
                "Category {} should have exactly 5 clues",
                category
            );

            for (row_idx, expected_val) in expected_values.iter().enumerate() {
                let clue_idx = cat_idx * 5 + row_idx;
                assert_eq!(clues[clue_idx].value, *expected_val);
                assert_eq!(clues[clue_idx].category, category);
            }
        }
    }

    #[test]
    fn test_clue_status_transitions() {
        let mut clue = initial_clues().remove(0);
        assert_eq!(clue.status, ClueStatus::Unanswered);

        clue.status = ClueStatus::Correct;
        assert_eq!(clue.status, ClueStatus::Correct);

        clue.status = ClueStatus::Incorrect;
        assert_eq!(clue.status, ClueStatus::Incorrect);
    }
}
