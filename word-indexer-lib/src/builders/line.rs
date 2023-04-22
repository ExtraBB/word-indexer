use crate::models::Word;

#[derive(Debug, PartialEq)]
pub struct Line {
    words: Vec<Word>,
    top: f64,
    bottom: f64,
}

fn overlap(a1: f64, a2: f64, b1: f64, b2: f64, threshold: f64) -> bool {
    let intersection = f64::max(0.0, f64::min(a2, b2) - f64::max(a1, b1));
    let union = a2 - a1 + b2 - b1 - intersection;
    return intersection / union > threshold;
}

pub fn build(words: Vec<Word>) -> Vec<Line> {
    let mut partial_lines: Vec<Line> = Vec::new();

    for word in words {
        match partial_lines
            .iter_mut()
            .find(|line| overlap(line.top, line.bottom, word.y, word.y + word.height, 0.9))
        {
            Some(line) => {
                line.top = f64::min(line.top, word.y);
                line.bottom = f64::max(line.bottom, word.y + word.height);
                line.words.push(word);
            }
            None => {
                partial_lines.push(Line {
                    bottom: word.y + word.height,
                    top: word.y,
                    words: vec![word],
                });
                continue;
            }
        }
    }

    return partial_lines
        .iter_mut()
        .map(|line| {
            let mut new_words = Vec::new();
            new_words.append(&mut line.words);
            new_words.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
            return Line {
                words: new_words,
                top: line.top,
                bottom: line.bottom,
            };
        })
        .collect();
}

#[cfg(test)]
pub mod integration_tests {
    use super::*;
    use rstest::rstest;

    fn create_test_word(text: String, x: f64, y: f64, width: f64, height: f64) -> Word {
        Word {
            text,
            x,
            y,
            width,
            height,
            angle: 0.0,
            characters: Vec::new(),
        }
    }

    #[macro_export]
    macro_rules! assert_eq_f64 {
        ($left:expr, $right:expr) => {{
            assert!(f64::abs($left - $right) < 0.0000001);
        }};
    }

    #[rstest]
    fn empty_words() {
        let actual = build(Vec::new());
        assert_eq!(0, actual.len());
    }

    #[rstest]
    fn single_word_single_line() {
        let word = create_test_word(String::from("word1"), 10.0, 10.0, 100.0, 10.0);
        let actual = build(vec![word]);

        assert_eq!(1, actual.len());
        assert_eq!(10.0, actual[0].top);
        assert_eq!(20.0, actual[0].bottom);
        assert_eq!("word1", &actual[0].words[0].text);
    }

    #[rstest]
    fn multiple_words_single_line_happy_case() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.0, 100.0, 10.0);
        let word2 = create_test_word(String::from("word2"), 110.0, 10.0, 200.0, 10.0);
        let word3 = create_test_word(String::from("word3"), 300.0, 10.0, 340.0, 10.0);
        let actual = build(vec![word1, word2, word3]);

        assert_eq!(1, actual.len());
        assert_eq!(10.0, actual[0].top);
        assert_eq!(20.0, actual[0].bottom);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);
        assert_eq!("word3", &actual[0].words[2].text);
    }

    #[rstest]
    fn multiple_words_single_line_wrong_order() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.0, 100.0, 10.0);
        let word2 = create_test_word(String::from("word2"), 300.0, 10.0, 340.0, 10.0);
        let word3 = create_test_word(String::from("word3"), 110.0, 10.0, 200.0, 10.0);
        let actual = build(vec![word1, word2, word3]);

        assert_eq!(1, actual.len());
        assert_eq!(10.0, actual[0].top);
        assert_eq!(20.0, actual[0].bottom);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[2].text);
        assert_eq!("word3", &actual[0].words[1].text);
    }

    #[rstest]
    fn multiple_words_single_line_not_exact() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.1, 100.0, 9.8);
        let word2 = create_test_word(String::from("word2"), 110.0, 9.9, 200.0, 10.3);
        let word3 = create_test_word(String::from("word3"), 300.0, 10.2, 340.0, 9.6);
        let actual = build(vec![word1, word2, word3]);

        assert_eq!(1, actual.len());
        assert_eq_f64!(9.9, actual[0].top);
        assert_eq_f64!(20.2, actual[0].bottom);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);
        assert_eq!("word3", &actual[0].words[2].text);
    }

    #[rstest]
    fn multiple_words_multiple_lines_far() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.1, 100.0, 9.8);
        let word2 = create_test_word(String::from("word2"), 110.0, 9.9, 200.0, 10.3);
        let word3 = create_test_word(String::from("word3"), 300.0, 50.2, 340.0, 9.6);
        let actual = build(vec![word1, word2, word3]);

        assert_eq!(2, actual.len());

        // Line 0
        assert_eq_f64!(9.9, actual[0].top);
        assert_eq_f64!(20.2, actual[0].bottom);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);

        // Line 1
        assert_eq_f64!(50.2, actual[1].top);
        assert_eq_f64!(59.8, actual[1].bottom);
        assert_eq!("word3", &actual[1].words[0].text);
    }

    #[rstest]
    fn multiple_words_multiple_lines_close() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.1, 100.0, 9.8);
        let word2 = create_test_word(String::from("word2"), 110.0, 9.9, 200.0, 10.3);
        let word3 = create_test_word(String::from("word3"), 300.0, 17.5, 340.0, 10.0);
        let actual = build(vec![word1, word2, word3]);

        assert_eq!(2, actual.len());

        // Line 0
        assert_eq_f64!(9.9, actual[0].top);
        assert_eq_f64!(20.2, actual[0].bottom);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);

        // Line 1
        assert_eq_f64!(17.5, actual[1].top);
        assert_eq_f64!(27.5, actual[1].bottom);
        assert_eq!("word3", &actual[1].words[0].text);
    }
}
