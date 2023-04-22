use crate::{indexers, models::Page};

pub fn index_amounts(page: Page) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    for word in page.words {
        let Some(parsed) = indexers::amount::index(word) else { continue; };
        result.push(parsed.try_into().unwrap());
    }

    return result;
}
