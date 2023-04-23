use rust_decimal::Decimal;
use rusty_money::{iso, Money};

use crate::models::Word;

fn extract_amounts(text: &str) -> Vec<String> {
    let mut start: i32 = -1;
    let mut end: i32 = -1;
    let mut negative_sign_before: i32 = -1;
    let mut result = Vec::new();

    for (i, c) in text.chars().enumerate() {
        if c.is_digit(10) {
            if start < 0 {
                start = if negative_sign_before >= 0 {
                    negative_sign_before
                } else {
                    i as i32
                };
                negative_sign_before = -1;
            }
            end = i as i32 + 1;
        } else if start != end {
            if c != ',' && c != '.' {
                let amount = text.get(start as usize..end as usize).unwrap().to_string();
                result.push(amount);
                start = -1;
                end = -1;
                negative_sign_before = -1;
            }
        } else if start < 0 {
            negative_sign_before = if c == '-' { i as i32 } else { -1 };
        }
    }

    if start != end {
        let amount = text.get(start as usize..end as usize).unwrap().to_string();
        result.push(amount);
    }

    return result;
}

pub fn parse_amounts(text: &str) -> Vec<Decimal> {
    // TODO: Different locales, different currencies

    let amounts = extract_amounts(text);
    return amounts
        .iter()
        .map(|text| Money::from_str(&text, iso::USD))
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap().amount().to_owned())
        .collect();
}

pub fn index_segment(word: Word) -> Option<Decimal> {
    // TODO: Return location with result (Round coordinates to 3 decimals for deterministic results)
    // TODO: Different locales, different currencies
    todo!("")
}

#[cfg(test)]
pub mod indexers_amount_tests {
    use crate::test_utils::create_test_word;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("")]
    #[case("abcde")]
    #[case(",.,")]
    #[case("   ")]
    #[case("these are some words!")]
    fn parse_amounts_no_amount(#[case] text: String) {
        let actual = parse_amounts(&text);
        assert_eq!(0, actual.len());
    }

    #[rstest]
    #[case("1", Decimal::from_str_exact("1.0").unwrap())]
    #[case("10.23", Decimal::from_str_exact("10.23").unwrap())]
    #[case("1,000,000.12", Decimal::from_str_exact("1000000.12").unwrap())]
    fn parse_amounts_with_single_amount(#[case] text: String, #[case] expected: Decimal) {
        let actual = parse_amounts(&text);
        assert_eq!(1, actual.len());
        assert_eq!(expected, actual[0]);
    }

    #[rstest]
    #[case("-1", -Decimal::from_str_exact("1.0").unwrap())]
    #[case("-10.23", -Decimal::from_str_exact("10.23").unwrap())]
    #[case("-1,000,000.12", -Decimal::from_str_exact("1000000.12").unwrap())]
    fn parse_amounts_with_single_negative_amount(#[case] text: String, #[case] expected: Decimal) {
        let actual = parse_amounts(&text);
        assert_eq!(1, actual.len());
        assert_eq!(expected, actual[0]);
    }

    #[rstest]
    #[case("a1b", Decimal::from_str_exact("1.0").unwrap())]
    #[case("before10.23", Decimal::from_str_exact("10.23").unwrap())]
    #[case("before-1,000,000.12after", -Decimal::from_str_exact("1000000.12").unwrap())]
    fn parse_amounts_with_single_amount_in_word(#[case] text: String, #[case] expected: Decimal) {
        let actual = parse_amounts(&text);
        assert_eq!(1, actual.len());
        assert_eq!(expected, actual[0]);
    }
}
