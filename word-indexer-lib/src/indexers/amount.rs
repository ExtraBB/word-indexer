use rust_decimal::Decimal;
use rusty_money::{iso, Money};

use crate::models::Word;

fn trim_non_digit(text: &str) -> String {
    let mut start: usize = 0;
    let mut end: usize = text.len();
    let mut start_done = false;
    let mut total_chars = 0;

    for c in text.chars() {
        total_chars += 1;

        if c.is_digit(10) {
            end = total_chars;
            start_done = true;
        }

        if !start_done {
            start += 1;
        }
    }

    let result: Vec<char> = text.chars().into_iter().collect();
    return result[start..end].into_iter().collect();
}

pub fn index(word: Word) -> Option<Decimal> {
    // TODO: Work on lines and divide lines into parsing units based on char distance
    // TODO: Return location with result (Round coordinates to 3 decimals for deterministic results)
    // TODO: Different locales, different currencies
    // TODO: Parse multiple amounts in 1 parsing unit

    let trimmed = trim_non_digit(&word.text);
    return match Money::from_str(&trimmed, iso::USD) {
        Ok(v) => Some(v.amount().to_owned()),
        Err(_) => None,
    };
}
