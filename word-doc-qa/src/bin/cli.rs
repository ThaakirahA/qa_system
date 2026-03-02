use std::env;
use word_doc_qa::data::loader::load_doc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cli <docx_path>");
        return;
    }
    let path = &args[1];
    let paragraphs = load_doc(path);

    for (i, p) in paragraphs.iter().enumerate() {
        println!("{}: {}", i + 1, p);
    }
}