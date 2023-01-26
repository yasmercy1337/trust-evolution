#![allow(dead_code)]

use rand::Rng;

const MAX_MUTATE: f64 = 0.05;
const DEFAULT_WEIGHT: f64 = 5.0;
const DEFAULT_BIAS: f64 = 1.0;

#[derive(Copy, Clone, Debug)]
pub struct Node {
    weight: f64,
    bias: f64,
}

impl Node {
    fn new() -> Self {
        Self {
            weight: DEFAULT_WEIGHT,
            bias: DEFAULT_BIAS,
        }
        .mutate()
    }

    fn eval(&self, inputs: &Vec<f64>) -> f64 {
        inputs
            .iter()
            .map(|input| self.weight * input + self.bias)
            .sum()
    }

    fn mutate(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.weight += (rng.gen::<f64>() - 0.5) * MAX_MUTATE * DEFAULT_WEIGHT;
        self.bias += (rng.gen::<f64>() - 0.5) * MAX_MUTATE * DEFAULT_BIAS;
        self
    }
}

pub struct Layer {
    size: usize,
    nodes: Vec<Node>,
}

impl Layer {
    fn new(size: usize) -> Self {
        Self {
            size,
            nodes: vec![Node::new(); size],
        }
    }

    fn eval(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.nodes.iter().map(|node| node.eval(&inputs)).collect()
    }

    fn mutate(self) -> Self {
        Self {
            size: self.size,
            nodes: self.nodes.iter().map(|node| node.mutate()).collect(),
        }
    }
}

pub struct Network {
    layers: Vec<Layer>,
    num_out: usize,
}

impl Network {
    #[allow(unused_variables)]
    fn new(depth: usize, width: usize, num_outputs: usize) -> Self {
        todo!()
    }

    fn spawn_child(&self) -> Self {
        todo!()
    }

    fn eval(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.reduce(
            self.num_out,
            self.layers
                .iter()
                .fold(inputs, |acc, layer| layer.eval(acc)),
        )
    }

    fn reduce(&self, out_len: usize, before: Vec<f64>) -> Vec<f64> {
        before
            .windows(before.len() - out_len + 1)
            .map(|arr| arr.iter().sum())
            .collect()
    }
}
