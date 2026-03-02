# Word Document Q&A System

## Overview
This project implements a transformer-based Q&A system on Word `.docx` calendar documents.  
It uses the `burn` deep learning framework (v0.20.1) and `docx-rs` (v0.4) for DOCX parsing.

## Folder Structure
word-doc-qa/
├── Cargo.toml
├── README.md
├── docs/report.md
├── src/
│ ├── lib.rs
│ ├── bin/train.rs
│ ├── bin/infer.rs
│ ├── data/
│ ├── model/
│ ├── training/
│ └── inference/
└── data_files/
├── calendar_2024.docx
├── calendar_2025.docx
└── calendar_2026.docx
## Usage

1. Place `.docx` files in `data_files/`.
2. Train the model:

```bash
cargo run --bin train


Run inference:

cargo run --bin infer

Notes

DO NOT change dependency versions; use exactly:

burn = 0.20.1

docx-rs = 0.4

tokenizers = 0.15

Cargo edition 2021 is required for grading compatibility.


---

✅ After this, your project is **fully structured and graded-ready**:

- All DOCX files go in:  
  `word-doc-qa/data_files/`
- Train with: `cargo run --bin train`
- Test inference with: `cargo run --bin infer`
- Maintains all required versions and edition.

---

