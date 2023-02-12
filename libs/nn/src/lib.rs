use rand::Rng;

pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

pub struct LayerConfig {
    pub neurons: usize,
}

impl Network {
    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        //fold takes initial state, then anonymous function including accumulator (which is previous state of array)
        // and also the next element of the array and performs the operation, and appends to the end
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }

    pub fn random(layers: &Vec<LayerConfig>) -> Self {
        // let mut built_layers = Vec::new();
        // //assert vs assert_eq! -- assert choose oepration, assert eq always equals
        // assert!(layers.len() > 1);

        // for adjacent_layers in layers.windows(2) {
        //     let input_neurons = adjacent_layers[0].neurons;
        //     let output_neurons = adjacent_layers[1].neurons;
        // }

        // built_layers.push(Layer::random(input_neurons, output_neurons));

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }
}

impl Layer {
    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // // create an array to store all outputs from this layer
        // let mut outputs = Vec::with_capacity(self.neurons.len());

        // // for each neuron on this layer, go through each one and collect outputs
        // // collect outputs and stor ethem into variable
        // for neuron in &self.neurons {
        //     let output = neuron.propogate(&inputs);
        //     outputs.push(output)
        // }

        // outputs
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }

    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        // let neurons = (0..output_neurons)
        //     .map(|_| Neuron::random(input_neurons))
        //     .collect();

        // Self { neurons }
        todo!();
    }
}

impl Neuron {
    fn propogate(&self, inputs: &Vec<f32>) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        // let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    mod random {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::Neuron;

        #[test]
        fn it_works() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            assert_eq!(neuron.bias, -0.6255188);
            assert_eq!(
                neuron.weights,
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807]
            );
        }
    }
}
