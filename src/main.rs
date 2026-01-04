use colored::Colorize;

use crate::{
    char_counter::CharCounter,
    dictionary::Dictionary,
    field::{FIELD_WIDTH, Field},
    word::{Word, create_word},
};

pub mod char_counter;
pub mod char_scorer;
pub mod dictionary;
pub mod field;
pub mod word;

pub fn get_score_and_new_field(
    old_field: &Field,
    dictionary: &Dictionary,
    on_hand_letters: &CharCounter,
    word: &Word,
) -> Result<(i32, Field), ()> {
    let mut new_field = old_field.clone();
    let new_letters_counter = new_field.try_add_word_and_get_new_letters_counter(word)?;
    if !new_letters_counter.is_less_than_or_eq(&on_hand_letters) {
        return Err(());
    }
    let score = new_field.get_horizontal_move_score(old_field, word, dictionary, new_letters_counter.sum())?;
    Ok((score, new_field))
}

pub fn solve(
    old_field: &Field,
    dictionary: &Dictionary,
    on_hand_letters: &CharCounter,
) -> (i32, Field) {
    let mut best_score = 0;
    let mut best_field = old_field.clone();
    for word_str in &dictionary.set {
        let word_letters = CharCounter::from_str(word_str);
        for i in 0..FIELD_WIDTH {
            let mut line_chars_counter = on_hand_letters.clone();
            for c in old_field.cells[i] {
                line_chars_counter.increment(c);
            }
            if !word_letters.is_less_than_or_eq(&line_chars_counter) {
                continue;
            }
            for j in 0..FIELD_WIDTH {
                let Ok(word) = create_word(word_str, i, j) else {
                    continue;
                };
                let Ok((score, new_field)) =
                    get_score_and_new_field(old_field, dictionary, on_hand_letters, &word)
                else {
                    continue;
                };
                if score > best_score {
                    best_score = score;
                    best_field = new_field;
                }
            }
        }
    }
    (best_score, best_field)
}

fn main() {
    let dictionary = Dictionary::read_from_file("russian_nouns.txt");
    let (mut old_field, on_hand_letters) = Field::read_from_file("field.txt");
    let (score1, new_field1) = solve(&old_field, &dictionary, &on_hand_letters);
    old_field.transpose();
    let (score2, mut new_field2) = solve(&old_field, &dictionary, &on_hand_letters);
    old_field.transpose();
    new_field2.transpose();
    let (score, new_field) = if score1 > score2 {
        (score1, new_field1)
    } else {
        (score2, new_field2)
    };
    println!("Score: {score}\n");
    for i in 0..FIELD_WIDTH {
        for j in 0..FIELD_WIDTH {
            let mut c = new_field.cells[i][j];
            if new_field.is_placeholder_char[i][j] {
                c = c.to_uppercase().next().unwrap();
            }
            if old_field.cells[i][j] == new_field.cells[i][j] {
                print!("{c} ");
            } else {
                print!("{} ", c.to_string().as_str().green())
            }
        }
        println!();
    }
}
