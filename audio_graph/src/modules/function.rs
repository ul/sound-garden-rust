//! # Functions
//!
//! Fn*N* modules allow to use regular numeric functions to transform input of *N* sources.
//!
//! Sources to connect: *N*, one for each argument of pure function.
use context::Context;
use module::Module;
use sample::{Frame, Sample};

pub struct Fn1 {
    ys: Vec<Sample>,
    f: fn(Sample) -> Sample,
}

impl Fn1 {
    pub fn new(ctx: &Context, f: fn(Sample) -> Sample) -> Box<Self> {
        Box::new(Fn1::raw(ctx, f))
    }

    pub fn raw(ctx: &Context, f: fn(Sample) -> Sample) -> Self {
        Fn1 {
            ys: vec![0.0; ctx.channels()],
            f,
        }
    }
}

impl Module for Fn1 {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, ctx: &mut Context, input: &Frame) {
        for i in 0..ctx.channels() {
            self.ys[i] = (self.f)(input[i]);
        }
    }
}

pub struct Fn2 {
    ys: Vec<Sample>,
    f: fn(Sample, Sample) -> Sample,
}

impl Fn2 {
    pub fn new(ctx: &Context, f: fn(Sample, Sample) -> Sample) -> Box<Self> {
        Box::new(Fn2 {
            ys: vec![0.0; ctx.channels()],
            f,
        })
    }
}

impl Module for Fn2 {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, ctx: &mut Context, input: &Frame) {
        let channels = ctx.channels();
        for i in 0..channels {
            self.ys[i] = (self.f)(input[i], input[i + channels]);
        }
    }
}

pub struct Fn3 {
    ys: Vec<Sample>,
    f: fn(Sample, Sample, Sample) -> Sample,
}

impl Fn3 {
    pub fn new(ctx: &Context, f: fn(Sample, Sample, Sample) -> Sample) -> Box<Self> {
        Box::new(Fn3 {
            ys: vec![0.0; ctx.channels()],
            f,
        })
    }
}

impl Module for Fn3 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, ctx: &mut Context, input: &Frame) {
        let channels = ctx.channels();
        for i in 0..channels {
            self.ys[i] = (self.f)(input[i], input[i + channels], input[i + 2 * channels]);
        }
    }
}
