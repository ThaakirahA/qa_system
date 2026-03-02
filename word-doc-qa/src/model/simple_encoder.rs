pub struct SimpleEncoder {
    pub input_size: usize,
    pub hidden_size: usize,
}

impl SimpleEncoder {
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        Self { input_size, hidden_size }
    }

    pub fn encode(&self, _x: Vec<f32>) -> Vec<f32> {
        // dummy encoding
        vec![0.0; self.hidden_size]
    }
}