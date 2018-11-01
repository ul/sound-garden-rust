use prelude::*;

pub struct Fn1 {
    ys: Vec<Sample>,
    f: fn(Sample) -> Sample,
}

impl Fn1 {
    pub fn new(ctx: &Context, f: fn(Sample) -> Sample) -> Box<Self> {
        Box::new(Fn1 {
            ys: vec![0.0; ctx.channels],
            f,
        })
    }
}

impl Module for Fn1 {
    fn output(&self) -> &[Sample] {
        &self.ys
    }

    fn sample(&mut self, ctx: &mut Context, input: &[Sample]) {
        for i in 0..ctx.channels {
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
            ys: vec![0.0; ctx.channels],
            f,
        })
    }
}

impl Module for Fn2 {
    fn output(&self) -> &[Sample] {
        &self.ys
    }

    fn sample(&mut self, ctx: &mut Context, input: &[Sample]) {
        let channels = ctx.channels;
        for i in 0..channels {
            self.ys[i] = (self.f)(input[i], input[i + channels]);
        }
    }
}
