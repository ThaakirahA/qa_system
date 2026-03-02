use word_doc_qa::data::loader::load_doc;
use word_doc_qa::model::transformer::build_encoder;

fn main() {
    let path = "example.docx";
    let paragraphs = load_doc(path);

    println!("Loaded {} paragraphs.", paragraphs.len());

    let encoder = build_encoder();
    println!("Transformer encoder built successfully.");
}