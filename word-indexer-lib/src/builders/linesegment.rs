use crate::models::Word;

use super::line::Line;

#[derive(Debug, PartialEq)]
pub struct LineSegment {
    words: Vec<Word>,
}

fn words_are_adjacent(a: &Word, b: &Word) -> bool {
    let distance = f64::abs(b.x - (a.x + a.width));
    return distance <= (a.width + b.width) / (a.text.len() + b.text.len()) as f64;
}

pub fn segment_line(line: Line) -> Vec<LineSegment> {
    let mut segments = Vec::new();

    let mut segment = LineSegment { words: Vec::new() };

    for word in line.words {
        let append = match segment.words.last() {
            Some(segment_word) => words_are_adjacent(segment_word, &word),
            None => true,
        };
        println!("append: {}", append);
        if append {
            segment.words.push(word);
        } else {
            segments.push(segment);
            segment = LineSegment { words: vec![word] };
        }
    }

    if segment.words.len() > 0 {
        segments.push(segment);
    }

    return segments;
}

#[cfg(test)]
pub mod line_segment_tests {
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

    #[rstest]
    fn segment_line_empty() {
        let actual = segment_line(Line {
            words: Vec::new(),
            top: 0.0,
            bottom: 0.0,
        });
        assert_eq!(0, actual.len());
    }

    #[rstest]
    fn segment_line_single_word() {
        let word = create_test_word(String::from("word1"), 10.0, 10.0, 100.0, 10.0);
        let line = Line {
            words: vec![word],
            top: 0.0,
            bottom: 0.0,
        };

        let actual = segment_line(line);

        assert_eq!(1, actual.len());
        assert_eq!("word1", &actual[0].words[0].text);
    }

    #[rstest]
    fn segment_line_single_segment() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.0, 50.0, 10.0);
        let word2 = create_test_word(String::from("word2"), 61.0, 10.0, 50.0, 10.0);
        let word3 = create_test_word(String::from("word3"), 112.0, 10.0, 50.0, 10.0);
        let line = Line {
            words: vec![word1, word2, word3],
            top: 0.0,
            bottom: 0.0,
        };

        let actual = segment_line(line);

        assert_eq!(1, actual.len());
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);
        assert_eq!("word3", &actual[0].words[2].text);
    }

    #[rstest]
    fn segment_line_multiple_segments() {
        let word1 = create_test_word(String::from("word1"), 10.0, 10.0, 50.0, 10.0);
        let word2 = create_test_word(String::from("word2"), 61.0, 10.0, 50.0, 10.0);
        let word3 = create_test_word(String::from("word3"), 130.0, 10.0, 50.0, 10.0);
        let word4 = create_test_word(String::from("word4"), 181.0, 10.0, 50.0, 10.0);
        let line = Line {
            words: vec![word1, word2, word3, word4],
            top: 0.0,
            bottom: 0.0,
        };

        let actual = segment_line(line);

        assert_eq!(2, actual.len());
        assert_eq!("word1", &actual[0].words[0].text);
        assert_eq!("word2", &actual[0].words[1].text);
        assert_eq!("word3", &actual[1].words[0].text);
        assert_eq!("word4", &actual[1].words[1].text);
    }
}
