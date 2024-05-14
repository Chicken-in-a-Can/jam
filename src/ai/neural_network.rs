type Vector = Vec<f64>;
type Matrix = Vec<Vector>;

pub struct Network{
    pub weights_hidden: Vector,
    pub biases_hidden: Vector,
    pub weights_output: Vector,
    pub biases_output: Vector,
    num_inputs: usize,
    num_hidden: usize,
    num_outputs: usize,
}

impl Network{
    pub fn new(
        num_inputs: usize,
        num_hidden: usize,
        num_outputs: usize,
        weights_hidden: Vector,
        biases_hidden: Vector,
        weights_output: Vector,
        biases_output: Vector
    ) -> Self{
        Self {
            weights_hidden,
            biases_hidden,
            weights_output,
            biases_output,
            num_inputs,
            num_hidden,
            num_outputs,
        }
    }
}
