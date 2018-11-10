//! # Noise
//!
//! White noise.
//!
//! Sources to connect: none required.
use module::Module;
use rand;
use sample::{Frame, Sample};

pub struct Noise {
    values: Vec<Sample>,
}

impl Noise {
    pub fn new(channels: usize) -> Self {
        Noise {
            values: vec![0.0; channels],
        }
    }
}

impl Module for Noise {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.values
    }

    fn sample(&mut self, _input: &Frame) {
        for value in self.values.iter_mut() {
            *value = rand::random();
        }
    }
}
