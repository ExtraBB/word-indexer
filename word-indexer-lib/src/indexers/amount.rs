use rust_decimal::Decimal;
use rusty_money::{iso, Money};

use crate::models::Word;

pub fn index(word: Word) -> Option<Decimal> {
    return match Money::from_str(&word.text, iso::USD) {
        Ok(v) => Some(v.amount().to_owned()),
        Err(_) => None,
    };
}
