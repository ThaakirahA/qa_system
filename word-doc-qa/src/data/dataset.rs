pub struct Dataset {
    pub inputs: Vec<Vec<f32>>,
    pub labels: Vec<i32>,
}

impl Dataset {
    pub fn new(inputs: Vec<Vec<f32>>, labels: Vec<i32>) -> Self {
        Self { inputs, labels }
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }
}