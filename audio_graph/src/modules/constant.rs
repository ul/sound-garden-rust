//! # Constant
//!
//! Constant module always outputs the same given sample in all channels.
//!
//! Sources to connect: none required.
use context::Context;
use module::Module;
use sample::{Frame, Sample};

pub struct Constant {
    values: Vec<Sample>,
}

impl Constant {
    pub fn new(ctx: &Context, x: Sample) -> Box<Self> {
        Box::new(Constant {
            values: vec![x; ctx.channels()],
        })
    }
}

impl Module for Constant {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.values
    }

    fn sample(&mut self, _ctx: &mut Context, _input: &Frame) {}
}
