pub struct ProfunctorOptic {
    pub name: String,
}

impl ProfunctorOptic {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }

    pub fn transform_tensor(&self, _tensor: &Vec<f64>) -> Vec<f64> {
        vec![0.0; 35]
    }
}
