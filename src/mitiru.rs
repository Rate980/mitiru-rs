use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteContent {
    pub character: String,
    pub quote: String,
    pub story: String,
    pub how_to_use: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotes {
    pub title: String,
    pub author: Option<String>,
    pub main: Vec<QuoteContent>,
    pub postscript: String,
}
