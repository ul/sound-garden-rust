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
use context::Context;
use module::Module;
use sample::{Frame, Sample};

pub struct Phasor {
    phases: Vec<Sample>,
}

impl Phasor {
    pub fn new(ctx: &Context) -> Box<Self> {
        Box::new(Phasor {
            phases: vec![0.0; ctx.channels()],
        })
    }
}

impl Module for Phasor {
    fn output(&self) -> &Frame {
        &self.phases
    }

    fn sample(&mut self, ctx: &mut Context, input: &Frame) {
        for (phase, frequency) in self.phases.iter_mut().zip(input.iter()) {
            let dx = frequency / ctx.sample_rate() as f64;
            *phase = ((*phase + dx + 1.0) % 2.0) - 1.0;;
        }
    }
}
