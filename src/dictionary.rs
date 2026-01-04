use std::{fs::File, io::Read, path::Path};

use fxhash::FxHashSet;

use crate::word::Word;

pub struct Dictionary {
    pub set: FxHashSet<String>,
}

impl Dictionary {
    pub fn read_from_file(path: impl AsRef<Path>) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let mut set = FxHashSet::default();
        for line in buf.trim().lines() {
            set.insert(line.trim().to_ascii_lowercase());
        }
        Self { set }
    }

    pub fn contains_word(&self, word: &Word) -> bool {
        let word_s: String = word.iter().map(|(_, _, c)| *c).collect();
        self.set.contains(&word_s)
    }
}
