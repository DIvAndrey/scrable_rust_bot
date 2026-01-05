use crate::{char_scorer::{LOOKUP_TABLE, MIN_LETTER_CODE}, field::EMPTY_CELL_CHAR};

#[derive(Clone)]
pub struct CharCounter {
    count: [u8; 34],
}

impl CharCounter {
    pub fn new() -> Self {
        Self { count: [0; 34] }
    }

    pub fn from_str(s: &str) -> Self {
        let mut counter = Self::new();
        for c in s.chars() {
            counter.increment(c);
        }
        counter
    }

    pub fn increment(&mut self, c: char) {
        if c == '-' || c == EMPTY_CELL_CHAR {
            return;
        }
        assert!('а' <= c && c <= 'я' || c == 'ё', "{c}");
        self.count[(c as u32 - MIN_LETTER_CODE) as usize] += 1;
    }

    pub fn is_less_than_or_eq(&self, other: &Self) -> bool {
        for i in 0..34 {
            if self.count[i] > other.count[i] {
                return false;
            }
        }
        true
    }

    pub fn sum(&self) -> i32 {
        self.count.iter().map(|&x| x as i32).sum()
    }

    pub fn score_sum(&self) -> i32 {
        let mut s = 0;
        for i in 0..34 {
            s += LOOKUP_TABLE[i] as i32 * self.count[i] as i32;
        }
        s
    }
}
