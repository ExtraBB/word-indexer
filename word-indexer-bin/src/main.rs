use word_indexer_lib::{
    models::{Character, Word},
    processor,
};

fn create_character_from_string(char: char) -> Character {
    return Character {
        unicode_data: char,
        x: 0.0,
        y: 0.0,
        width: 5.0,
        height: 10.0,
        angle: 0.0,
    };
}

fn create_word_from_string(word: &str) -> Word {
    return Word {
        text: String::from(word),
        x: 0.0,
        y: 0.0,
        width: word.len() as f64 * 5.0,
        height: 10.0,
        angle: 0.0,
        characters: word.chars().map(create_character_from_string).collect(),
    };
}

fn create_words_from_sentence(sentence: &str) -> Vec<Word> {
    return sentence
        .split_whitespace()
        .map(create_word_from_string)
        .collect();
}

fn main() {
    let words: Vec<Word> = create_words_from_sentence(
        "Hello I am 12.4 years old and have 1,000.25 euros on 10-12-2022.",
    );

    dbg!(processor::index_amounts(words));
}

#[cfg(test)]
pub mod integration_tests {
    use rstest::rstest;
    use word_indexer_lib::{models::Word, processor};

    #[rstest]
    #[case("This has 1,003.28 as the single number", vec![1_003.28])]
    #[case("This has 1,003.28; as the single number with trailing punctuation", vec![1_003.28])]
    #[case("This has 1,003.28 and 837,291.37 as multiple numbers", vec![1_003.28, 837_291.37])]
    #[case("This has some1,003.28within text", vec![1_003.28])]
    fn monetary_amount(#[case] sentence: &str, #[case] expected: Vec<f64>) {
        let words: Vec<Word> = crate::create_words_from_sentence(sentence);
        let actual = processor::index_amounts(words);
        assert_eq!(expected, actual);
    }
}
