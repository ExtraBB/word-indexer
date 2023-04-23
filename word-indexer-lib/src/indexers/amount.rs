use rust_decimal::Decimal;
use rusty_money::{iso, Money};

use crate::models::Word;

fn trim_non_digit(text: &str) -> String {
    let mut start: usize = 0;
    let mut end: usize = text.len();
    let mut start_done = false;
    let mut total_chars = 0;

    let mut previousChar: char = ' ';

    for c in text.chars() {
        total_chars += 1;

        if c.is_digit(10) {
            end = total_chars;

            if !start_done {
                start_done = true;
                if previousChar == '-' {
                    start -= 1;
                }
            }
        }

        if !start_done {
            start += 1;
        }

        previousChar = c.clone();
    }

    let result: Vec<char> = text.chars().into_iter().collect();
    return result[start..end].into_iter().collect();
}

pub fn index_word(word: &Word) -> Option<Decimal> {
    // TODO: Work on lines and divide lines into parsing units based on char distance
    // TODO: Return location with result (Round coordinates to 3 decimals for deterministic results)
    // TODO: Different locales, different currencies
    // TODO: Parse multiple amounts in 1 parsing unit
    // TODO: negative amounts

    let trimmed = trim_non_digit(&word.text);
    return match Money::from_str(&trimmed, iso::USD) {
        Ok(v) => Some(v.amount().to_owned()),
        Err(_) => None,
    };
}

pub fn index_segment(word: Word) -> Option<Decimal> {
    // TODO: Return location with result (Round coordinates to 3 decimals for deterministic results)
    // TODO: Different locales, different currencies

    let trimmed = trim_non_digit(&word.text);
    return match Money::from_str(&trimmed, iso::USD) {
        Ok(v) => Some(v.amount().to_owned()),
        Err(_) => None,
    };
}

#[cfg(test)]
pub mod indexers_amount_tests {
    use crate::{assert_eq_f64, models::Character, test_utils::create_test_word};

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("")]
    #[case("abcde")]
    #[case(",.,")]
    #[case("   ")]
    #[case("these are some words!")]
    fn index_word_no_amount(#[case] text: String) {
        let word = create_test_word(text, 0.0, 0.0, 0.0, 0.0, 0.0);
        let actual = index_word(&word);
        assert_eq!(None, actual);
    }

    #[rstest]
    #[case("1", Decimal::from_str_exact("1.0").unwrap())]
    #[case("10.23", Decimal::from_str_exact("10.23").unwrap())]
    #[case("1,000,000.12", Decimal::from_str_exact("1000000.12").unwrap())]
    fn index_word_with_amount(#[case] text: String, #[case] expected: Decimal) {
        let word = create_test_word(text, 0.0, 0.0, 0.0, 0.0, 0.0);
        match index_word(&word) {
            Some(actual) => assert_eq!(expected, actual),
            None => assert!(false),
        }
    }

    #[rstest]
    #[case("-1", -Decimal::from_str_exact("1.0").unwrap())]
    #[case("-10.23", -Decimal::from_str_exact("10.23").unwrap())]
    #[case("-1,000,000.12", -Decimal::from_str_exact("1000000.12").unwrap())]
    fn index_word_with_negative_amount(#[case] text: String, #[case] expected: Decimal) {
        let word = create_test_word(text, 0.0, 0.0, 0.0, 0.0, 0.0);
        match index_word(&word) {
            Some(actual) => assert_eq!(expected, actual),
            None => assert!(false),
        }
    }
}
