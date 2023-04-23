use crate::models::{Character, Word};

pub fn create_test_word(text: String, x: f64, y: f64, width: f64, height: f64, angle: f64) -> Word {
    let average_char_width = width / text.len() as f64;
    Word {
        text: text.clone(),
        x,
        y,
        width,
        height,
        angle,
        characters: text
            .chars()
            .enumerate()
            .map(|(i, unicode_data)| Character {
                unicode_data,
                x: x + i as f64 * average_char_width,
                y,
                width: average_char_width,
                height,
                angle,
            })
            .collect(),
    }
}
