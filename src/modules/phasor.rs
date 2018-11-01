use prelude::*;

pub struct Phasor {
    phases: Vec<Sample>,
}

impl Phasor {
    pub fn new(ctx: &Context) -> Box<Self> {
        Box::new(Phasor {
            phases: vec![0.0; ctx.channels],
        })
    }
}

impl Module for Phasor {
    fn output(&self) -> &[Sample] {
        &self.phases
    }

    fn sample(&mut self, ctx: &mut Context, input: &[Sample]) {
        for (phase, frequency) in self.phases.iter_mut().zip(input.iter()) {
            let dx = frequency / ctx.sample_rate as f64;
            *phase = ((*phase + dx + 1.0) % 2.0) - 1.0;;
        }
    }
}
