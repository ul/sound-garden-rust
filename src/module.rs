use prelude::*;

pub trait Module {
    fn output(&self) -> &[Sample];
    fn sample(&mut self, ctx: &mut Context, input: &[Sample]);
}
