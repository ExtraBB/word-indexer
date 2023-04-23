use crate::{indexers, models::Word};

pub fn index_amounts(words: Vec<Word>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    for word in words {
        let Some(parsed) = indexers::amount::index(word) else { continue; };
        result.push(parsed.try_into().unwrap());
    }

    return result;
}
