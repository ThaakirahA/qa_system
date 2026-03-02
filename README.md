# Word Document Q&A System

**SEG 580S: Software Engineering Deep Learning Systems Training on CPUT Data**  
**Project:** Question-Answering System with Rust and Burn Framework  

---

## Project Overview

This project implements a **Question-Answering (Q&A) system** that reads Word documents (`.docx`) and answers questions about their content. The system is built using **Rust** and the **Burn deep learning framework**, demonstrating a complete machine learning pipeline:

- Document processing (`.docx`)
- Tokenization and batching
- Transformer-based neural network training
- Command-line interface for Q&A
- Model deployment and inference

---

## Repository Structure


word-doc-qa/
├── src/ # Rust source code
├── Cargo.toml # Project configuration
├── report.md # Project report in Markdown
├── README.md # This file
└── examples/ # Example Word documents (optional)


---

## Setup Instructions

1. **Clone the repository:**

```bash
git clone https://github.com/<username>/word-doc-qa.git
cd word-doc-qa

Install Rust (if not installed):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Build the project:

cargo build --release

Run tests (optional):

cargo test
Usage

Run the system from the command line:

cargo run --release -- <path_to_docx_file> --question "Your question here"

Example:

cargo run --release -- examples/graduation_schedule.docx --question "When is the 2026 End-of-Year Graduation Ceremony?"

The system will output the predicted answer based on the document content.

Example Questions

When is the 2026 End-of-Year Graduation Ceremony?

How many times did HDC hold meetings in 2024?

[Add your other test questions here]

Report

Full project report is available in Markdown format:

report.md

This includes:

Problem statement and motivation

Implementation details (architecture, data pipeline, training)

Experiments and results

Conclusion and future work

Dependencies

The project uses the following Rust crates:

Crate	Version	Purpose
burn	0.20.1	Deep learning framework
docx-rs	0.4	Load .docx documents
tokenizers	0.15	Tokenization
serde	1.0	Serialization
serde_json	1.0	JSON handling

Note: Do not change the versions; grading depends on them.
