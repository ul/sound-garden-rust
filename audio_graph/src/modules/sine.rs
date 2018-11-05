//! # Sine wave
//!
//! Sources to connect: frequency.
use context::Context;
use module::Module;
use modules::function::Fn1;
use modules::phasor::Phasor;
use pure::sine;
use sample::Frame;

pub struct Sine {
    phasor: Phasor,
    osc: Fn1,
}

impl Sine {
    pub fn new(ctx: &Context) -> Box<Self> {
        let phasor = Phasor::raw(ctx);
        let osc = Fn1::raw(ctx, sine);
        Box::new(Sine { phasor, osc })
    }
}

impl Module for Sine {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        self.osc.output()
    }

    fn sample(&mut self, ctx: &mut Context, input: &Frame) {
        self.phasor.sample(ctx, input);
        self.osc.sample(ctx, self.phasor.output());
    }
}
