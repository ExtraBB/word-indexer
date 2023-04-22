use crate::models::Word;

pub fn index(word: Word) -> Option<f64> {
    return match word.text.parse() {
        Ok(v) => Some(v),
        Err(_) => None,
    };
}
