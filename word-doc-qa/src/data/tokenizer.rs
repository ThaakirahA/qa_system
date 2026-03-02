pub struct DocTokenizer;

impl DocTokenizer {
    pub fn new() -> Self { Self }

    pub fn encode(&self, text: &str) -> Vec<i64> {
        // dummy tokenization: each word = 1 token
        text.split_whitespace().map(|_| 1).collect()
    }
}