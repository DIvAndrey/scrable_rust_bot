use std::{fs::File, io::Read, mem::swap, path::Path};

use crate::{
    char_counter::CharCounter, char_scorer::get_char_score, dictionary::Dictionary, word::Word,
};

pub const FIELD_WIDTH: usize = 15;
pub const EMPTY_CELL_CHAR: char = '.';

pub const LETTER_MULTIPLIERS: [[i32; FIELD_WIDTH]; FIELD_WIDTH] = [
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 3, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1, 3, 1],
    [1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 1],
    [1, 3, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1, 3, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2],
    [1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
];

pub const WORD_MULTIPLIERS: [[i32; FIELD_WIDTH]; FIELD_WIDTH] = [
    [3, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 3],
    [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1],
    [1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1],
    [3, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 3],
];

#[derive(Clone, Debug)]
pub struct Field {
    pub cells: [[char; FIELD_WIDTH]; FIELD_WIDTH],
    pub is_placeholder_char: [[bool; FIELD_WIDTH]; FIELD_WIDTH],
}

impl Field {
    pub fn read_from_file(path: impl AsRef<Path>) -> (Self, CharCounter) {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let lines: Vec<&str> = buf.trim().lines().collect();
        let mut cells = [[EMPTY_CELL_CHAR; FIELD_WIDTH]; FIELD_WIDTH];
        let mut is_placeholder_char = [[false; FIELD_WIDTH]; FIELD_WIDTH];
        for (i, &line) in lines.iter().take(FIELD_WIDTH).enumerate() {
            for (j, char_str) in line.trim().split_whitespace().enumerate() {
                assert_eq!(char_str.chars().count(), 1, "{}", char_str);
                let mut c = char_str.chars().next().unwrap();
                if c.is_uppercase() {
                    is_placeholder_char[i][j] = true;
                    c = c.to_lowercase().next().unwrap();
                }
                cells[i][j] = c;
            }
        }
        (
            Self {
                cells,
                is_placeholder_char,
            },
            CharCounter::from_str(lines.last().unwrap()),
        )
    }

    pub fn transpose(&mut self) {
        for i in 0..FIELD_WIDTH {
            for j in 0..i {
                let [line_i, line_j] = self.cells.get_disjoint_mut([i, j]).unwrap();
                swap(&mut line_i[j], &mut line_j[i]);
                let [line_i, line_j] = self.is_placeholder_char.get_disjoint_mut([i, j]).unwrap();
                swap(&mut line_i[j], &mut line_j[i]);
            }
        }
    }

    fn is_word_isolated(&self, word: &Word) -> bool {
        for &(i, j, _) in word {
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;
                if ni >= FIELD_WIDTH || nj >= FIELD_WIDTH {
                    continue;
                }
                if self.cells[ni][nj] != EMPTY_CELL_CHAR {
                    return false;
                }
            }
        }
        true
    }

    pub fn try_add_word_and_get_new_letters_counter(
        &mut self,
        word: &Word,
    ) -> Result<CharCounter, ()> {
        if self.is_word_isolated(word) {
            return Err(());
        }
        let mut counter = CharCounter::new();
        for &(i, j, c) in word {
            let old_c = self.cells[i][j];
            if old_c == EMPTY_CELL_CHAR {
                self.cells[i][j] = c;
                counter.increment(c);
            } else if old_c != c {
                return Err(());
            }
        }
        Ok(counter)
    }

    fn get_word_score(&self, old_field: &Self, word: &Word) -> Result<i32, ()> {
        let mut score = 0;
        let mut word_score_multiplier = 1;
        for &(i, j, new_c) in word {
            let old_c = old_field.cells[i][j];
            assert_eq!(new_c, self.cells[i][j]);
            assert_ne!(new_c, EMPTY_CELL_CHAR);
            if self.is_placeholder_char[i][j] {
                continue;
            }
            let mut char_score = get_char_score(new_c);
            if old_c != new_c {
                char_score *= LETTER_MULTIPLIERS[i][j];
                word_score_multiplier *= WORD_MULTIPLIERS[i][j];
            }
            score += char_score;
        }
        score *= word_score_multiplier;
        Ok(score)
    }

    pub fn select_horizontal_word(
        &self,
        word_i: usize,
        j1: usize,
        j2: usize,
    ) -> Vec<(usize, usize, char)> {
        let mut v = Vec::with_capacity(j2 - j1 + 1);
        for j in j1..=j2 {
            v.push((word_i, j, self.cells[word_i][j]));
        }
        return v;
    }

    pub fn select_vertical_word(
        &self,
        i1: usize,
        i2: usize,
        word_j: usize,
    ) -> Vec<(usize, usize, char)> {
        let mut v = Vec::with_capacity(i2 - i1 + 1);
        for i in i1..=i2 {
            v.push((i, word_j, self.cells[i][word_j]));
        }
        return v;
    }

    pub fn get_horizontal_move_score(
        &self,
        old_field: &Self,
        word: &Word,
        dictionary: &Dictionary,
        used_letters_num: i32,
    ) -> Result<i32, ()> {
        if !dictionary.contains_word(word) {
            return Err(());
        }

        let mut score = 0;

        let word_i = word.first().unwrap().0;
        let word_j1 = word.first().unwrap().1;
        let word_j2 = word.last().unwrap().1;

        if used_letters_num == 0 {
            return Err(());
        }
        assert!(used_letters_num >= 1);
        assert!(used_letters_num <= 7);
        if used_letters_num == 7 {
            score += 50;
        }

        // Vertical words
        for &(_i, j, new_c) in word {
            assert_eq!(_i, word_i);
            assert_eq!(new_c, self.cells[word_i][j]);

            if old_field.cells[word_i][j] == new_c {
                continue;
            }

            let mut min_i = word_i;
            for curr_i in (0..word_i).rev() {
                if self.cells[curr_i][j] == EMPTY_CELL_CHAR {
                    break;
                }
                min_i = curr_i;
            }
            let mut max_i = word_i;
            for curr_i in word_i..FIELD_WIDTH {
                if self.cells[curr_i][j] == EMPTY_CELL_CHAR {
                    break;
                }
                max_i = curr_i;
            }
            if min_i != max_i {
                // If len > 1
                let curr_word = self.select_vertical_word(min_i, max_i, j);
                if !dictionary.contains_word(&curr_word) {
                    return Err(());
                }
                score += self.get_word_score(old_field, &curr_word)?;
            }
        }

        // Horizontal word
        let mut min_j = word_j1;
        for j in (0..word_j1).rev() {
            if self.cells[word_i][j] == EMPTY_CELL_CHAR {
                break;
            }
            min_j = j;
        }
        let mut max_j = word_j2;
        for j in word_j2..FIELD_WIDTH {
            if self.cells[word_i][j] == EMPTY_CELL_CHAR {
                break;
            }
            max_j = j;
        }
        let curr_word = self.select_horizontal_word(word_i, min_j, max_j);
        if !dictionary.contains_word(&curr_word) {
            return Err(());
        }
        score += self.get_word_score(old_field, &curr_word)?;

        Ok(score)
    }
}
