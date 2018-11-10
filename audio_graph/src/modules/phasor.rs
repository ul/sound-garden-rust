//! # Phasor
//!
//! ```
//!  1     /|    /|    /|    /|
//!       / |   / |   / |   / |
//!  0   /  |  /  |  /  |  /  |
//!     /   | /   | /   | /   |
//! -1 /    |/    |/    |/    |
//! ```
//!
//! Phasor module generates a saw wave in the range -1..1.
//! Frequency is controlled by the input for each channel separately and can be variable.
//!
//! It is called phasor because it could be used as input phase for other oscillators, which become
//! just pure transformations then and are not required to care about handling varying frequency by
//! themselves anymore.
//!
//! Sources to connect: frequency.
use module::Module;
use sample::{Frame, Sample};

pub struct Phasor {
    phases: Vec<Sample>,
    sample_rate: Sample,
}

impl Phasor {
    pub fn new(channels: usize, sample_rate: usize) -> Self {
        Phasor {
            phases: vec![0.0; channels],
            sample_rate: sample_rate as Sample,
        }
    }
}

impl Module for Phasor {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.phases
    }

    fn sample(&mut self, input: &Frame) {
        for (phase, frequency) in self.phases.iter_mut().zip(input.iter()) {
            let dx = frequency / self.sample_rate;
            *phase = ((*phase + dx + 1.0) % 2.0) - 1.0;;
        }
    }
}
