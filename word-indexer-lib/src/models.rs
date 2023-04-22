#[derive(Debug, PartialEq)]
pub struct Character {
    pub unicode_data: char,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
}

#[derive(Debug, PartialEq)]
pub struct Word {
    pub text: String, // Optimize out to reduce memory usage
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
    pub characters: Vec<Character>,
}

// todo remove
#[derive(Debug, PartialEq)]
pub struct Page {
    pub page_number: u32,
    pub words: Vec<Word>,
}
