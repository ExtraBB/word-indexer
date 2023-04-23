use crate::models::Word;

use super::line::Line;

#[derive(Debug, PartialEq)]
pub struct LineSegment {
    pub words: Vec<Word>,
    pub text: String,
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

fn words_are_adjacent(a: &Word, b: &Word) -> bool {
    // Consider words adjacent if less than 2 character widths apart
    let distance = f64::abs(b.x - (a.x + a.width));
    return distance <= (a.width + b.width) / (a.text.len() + b.text.len()) as f64 * 2.0;
}

pub fn partition(line: Line) -> Vec<LineSegment> {
    let mut segments = Vec::new();

    let mut segment = LineSegment {
        words: Vec::new(),
        text: String::new(),
        top: line.top,
        bottom: line.bottom,
        left: 0.0,
        right: 0.0,
    };

    for word in line.words {
        let append = match segment.words.last() {
            Some(segment_word) => words_are_adjacent(segment_word, &word),
            None => true,
        };

        let left = word.x;
        let right = word.x + word.width;

        if append {
            segment.right = right;
            segment.text.push_str(&word.text);
            segment.words.push(word);
        } else {
            segments.push(segment);
            segment = LineSegment {
                text: String::from(&word.text),
                words: vec![word],
                top: line.top,
                bottom: line.bottom,
                left,
                right,
            };
        }

        if segment.words.len() == 1 {
            segment.left = left;
        }
    }

    if segment.words.len() > 0 {
        segments.push(segment);
    }

    return segments;
}

#[cfg(test)]
pub mod partitioners_line_segment_tests {
    use crate::test_utils::create_test_word;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn partition_empty() {
        let actual = partition(Line {
            words: Vec::new(),
            top: 0.0,
            bottom: 0.0,
        });
        assert_eq!(0, actual.len());
    }

    #[rstest]
    fn partition_single_word() {
        let word = create_test_word(String::from("word1"), 10.0, 10.0, 100.0, 10.0, 0.0);
        let line = Line {
            words: vec![word],
            top: 0.0,
            bottom: 0.0,
        };

        let actual = partition(line);

        assert_eq!(1, actual.len());
        assert_eq!(10.0, actual[0].left);
        assert_eq!(110.0, actual[0].right);
        assert_eq!("word1", actual[0].text);
        assert_eq!("word1", &actual[0].words[0].text);
    }

    #[rstest]
    fn partition_single_segment() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.0, 50.0, 10.0, 0.0);
        let word2 = create_test_word(String::from("word2"), 61.0, 10.0, 50.0, 10.0, 0.0);
        let word3 = create_test_word(String::from("word3"), 112.0, 10.0, 50.0, 10.0, 0.0);
        let line = Line {
            words: vec![word1, word2, word3],
            top: 0.0,
            bottom: 0.0,
        };

        let actual = partition(line);

        assert_eq!(1, actual.len());
        assert_eq!(10.0, actual[0].left);
        assert_eq!(162.0, actual[0].right);
        assert_eq!("word1word2word3", actual[0].text);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);
        assert_eq!("word3", &actual[0].words[2].text);
    }

    #[rstest]
    fn partition_multiple_segments() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.0, 50.0, 10.0, 0.0);
        let word2 = create_test_word(String::from("word2"), 61.0, 10.0, 50.0, 10.0, 0.0);
        let word3 = create_test_word(String::from("word3"), 140.0, 10.0, 50.0, 10.0, 0.0);
        let word4 = create_test_word(String::from("word4"), 191.0, 10.0, 50.0, 10.0, 0.0);
        let line = Line {
            words: vec![word1, word2, word3, word4],
            top: 0.0,
            bottom: 0.0,
        };

        let actual = partition(line);

        assert_eq!(2, actual.len());

        // Segment 1
        assert_eq!(10.0, actual[0].left);
        assert_eq!(111.0, actual[0].right);
        assert_eq!("word1word2", actual[0].text);
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);

        // Segment 2
        assert_eq!(140.0, actual[1].left);
        assert_eq!(241.0, actual[1].right);
        assert_eq!("word3word4", actual[1].text);
        assert_eq!("word3", &actual[1].words[0].text);
        assert_eq!("word4", &actual[1].words[1].text);
    }
}
