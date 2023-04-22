use word_indexer_lib::{
    models::Page,
    models::{Character, Word},
    processor,
};

fn create_character_from_string(char: &char) -> Character {
    return Character {
        unicode_data: char.to_string(),
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
        characters: word
            .chars()
            .map(|s| create_character_from_string(&s))
            .collect(),
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

    let page: Page = Page {
        page_number: 1,
        words,
    };

    dbg!(processor::index_amounts(page));
}
