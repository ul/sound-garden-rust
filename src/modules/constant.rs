use prelude::*;

pub struct Constant {
    values: Vec<Sample>,
}

impl Constant {
    pub fn new(ctx: &Context, x: Sample) -> Box<Self> {
        Box::new(Constant {
            values: vec![x; ctx.channels],
        })
    }
}

impl Module for Constant {
    fn output(&self) -> &[Sample] {
        &self.values
    }

    fn sample(&mut self, _ctx: &mut Context, _input: &[Sample]) {}
}
