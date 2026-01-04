use crate::field::FIELD_WIDTH;

pub type Word = Vec<(usize, usize, char)>;

pub fn create_word(content: &str, start_i: usize, start_j: usize) -> Result<Word, ()> {
    let mut v = Vec::with_capacity(content.len());
    let mut j = start_j;
    for c in content.chars() {
        if j >= FIELD_WIDTH {
            return Err(());
        }
        v.push((start_i, j, c));
        j += 1;
    }
    Ok(v)
}
