use rust_decimal::Decimal;

use crate::{indexers, models::Word, partitioners};

pub fn index_amounts(words: Vec<Word>) -> Vec<Decimal> {
    let mut result: Vec<Decimal> = Vec::new();

    let lines = partitioners::line::partition(words);

    for line in lines {
        let line_segments = partitioners::line_segment::partition(line);
        for line_segment in line_segments {
            // Todo parse segment as a whole
            for word in line_segment.words {
                let mut amounts = indexers::amount::parse_amounts(&word.text);
                result.append(&mut amounts);
            }
        }
    }

    return result;
}
