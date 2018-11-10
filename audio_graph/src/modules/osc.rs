//! # Oscillator
//!
//! Sources to connect: frequency.
use module::Module;
use modules::function::Fn1;
use modules::phasor::Phasor;
use sample::{Frame, Sample};

pub struct Osc {
    phasor: Phasor,
    osc: Fn1,
}

impl Osc {
    pub fn new(channels: usize, sample_rate: usize, f: fn(Sample) -> Sample) -> Self {
        let phasor = Phasor::new(channels, sample_rate);
        let osc = Fn1::new(channels, f);
        Osc { phasor, osc }
    }
}

impl Module for Osc {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        self.osc.output()
    }

    fn sample(&mut self, input: &Frame) {
        self.phasor.sample(input);
        self.osc.sample(self.phasor.output());
    }
}
