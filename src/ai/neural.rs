use crate::ai::neuron::Neuron;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Neural {
    neurons: Vec<Vec<Neuron>>,
}

impl Neural {
    pub fn new(
        input_layers_num: usize,
        layers_num: usize,
        layers_size: usize,
        output_layers_num: usize,
    ) -> Self {
        Self {
            neurons: (0..layers_num + 1)
                .map(|layer_num| {
                    if layer_num == layers_num {
                        (0..output_layers_num)
                            .map(|_| Neuron::new(layers_num))
                            .collect()
                    } else {
                        (0..layers_size)
                            .map(|_| {
                                if layer_num == 0 {
                                    Neuron::new(input_layers_num)
                                } else {
                                    Neuron::new(layers_size)
                                }
                            })
                            .collect()
                    }
                })
                .collect(),
        }
    }

    pub fn execute(&self, data: Vec<f64>) -> Vec<f64> {
        let mut input = data.clone();
        for layer in self.neurons.iter() {
            let mut result = Vec::new();

            for neuron in layer.iter() {
                result.push(neuron.execute(&input));
            }

            input.clear();
            input.extend(result);
        }
        input
    }

    pub fn mutate(&self) -> Self {
        let mut new_brain = self.clone();
        let layers_num = new_brain.neurons.len();
        let mutation_layer = new_brain
            .neurons
            .get_mut(rand::thread_rng().gen_range(0..layers_num))
            .unwrap();
        let layers_size = mutation_layer.len();
        let mutation_neuron = mutation_layer
            .get_mut(rand::thread_rng().gen_range(0..layers_size))
            .unwrap();
        mutation_neuron.mutate();
        new_brain
    }
}
