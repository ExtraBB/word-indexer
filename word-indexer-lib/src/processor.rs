use crate::{indexers, models::Word, partitioners};

pub fn index_amounts(words: Vec<Word>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    let lines = partitioners::line::partition(words);

    for line in lines {
        let line_segments = partitioners::line_segment::partition(line);
        for line_segment in line_segments {
            // Todo parse segment as a whole
            for word in line_segment.words {
                let Some(parsed) = indexers::amount::index_word(word) else { continue; };
                result.push(parsed.try_into().unwrap());
            }
        }
    }

    return result;
}
