use crate::data::dataset::QADataset;
use crate::model::transformer::QATransformer;
use burn::tensor::backend::NdArrayBackend;
use burn::optim::Adam;
use burn::tensor::ops::cross_entropy;

pub struct Trainer {
    pub model: QATransformer<NdArrayBackend>,
    pub optimizer: Adam<NdArrayBackend>,
}

impl Trainer {
    pub fn new(model: QATransformer<NdArrayBackend>, lr: f32) -> Self {
        let optimizer = Adam::new(&model, lr);
        Self { model, optimizer }
    }

    pub fn train(&mut self, dataset: &QADataset, epochs: usize) {
        for _ in 0..epochs {
            for i in 0..dataset.len() {
                let (inputs, labels) = dataset.get_tensor(i);
                let logits = self.model.forward(inputs.into());
                let loss = cross_entropy(&logits, &labels.into());
                self.optimizer.backward_step(&mut self.model, &loss);
            }
        }
    }
}