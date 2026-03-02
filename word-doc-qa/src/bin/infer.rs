use word_doc_qa::data::dataset::QADataset;
use word_doc_qa::data::loader::load_calendar_docs;
use word_doc_qa::data::tokenizer::DocTokenizer;
use word_doc_qa::model::transformer::QATransformer;
use burn::tensor::backend::NdArrayBackend;

fn main() {
    let tokenizer = DocTokenizer::new();
    let paragraphs = load_calendar_docs();
    let dataset = QADataset::new(tokenizer, paragraphs);

    let model = QATransformer::<NdArrayBackend>::new(128, 8, 2);

    // Example Q&A: simply pick first document
    if dataset.len() > 0 {
        let (input, mask) = dataset.get_tensor(0);
        let output = model.forward(input.into());
        println!("Q&A inference output tensor: {:?}", output);
    }
}