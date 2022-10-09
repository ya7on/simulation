use rand::Rng;

#[derive(Debug, Clone)]
pub struct Neuron {
    bias: f64,
    weights: Vec<f64>,
}

impl Neuron {
    pub fn new(prev_layer_size: usize) -> Self {
        Self {
            bias: rand::thread_rng().gen_range(-1.0..1.0),
            weights: (0..prev_layer_size)
                .map(|_| rand::thread_rng().gen_range(-1.0..1.0))
                .collect(),
        }
    }

    pub fn execute(&self, data: &Vec<f64>) -> f64 {
        let mut x = 0.0;
        for i in 0..data.len() {
            x += data.get(i).unwrap() * self.weights.get(i).unwrap();
        }
        x += self.bias;
        let e = std::f64::consts::E;
        1.0 / (1.0 + e.powf(-x))
    }

    pub fn mutate(&mut self) {
        self.bias = rand::thread_rng().gen_range(-1.0..1.0);
        self.weights = (0..self.weights.len())
            .map(|_| rand::thread_rng().gen_range(-1.0..1.0))
            .collect();
    }
}
