mod data;
mod model;
mod train;

use data::dataset::QADataset;
use data::loader::load_calendar_docs;
use data::tokenizer::DocTokenizer;

fn main() {
    println!("🔥 Loading DOCX files...");
    let docs = load_calendar_docs();

    println!("Loaded {} documents.", docs.len());

    let tokenizer = DocTokenizer::new();
    let dataset = QADataset::new(tokenizer, docs);

    println!("Dataset size: {}", dataset.len());

    println!("Training starting...");
}