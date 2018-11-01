extern crate cpal;
extern crate fixedbitset;
extern crate petgraph;

mod audio;
mod context;
mod graph;
mod module;
mod modules;
mod prelude;
mod pure;
mod sugar;

use prelude::*;

fn main() {
    let (event_loop, ctx) = audio::init();
    let mut g = AudioGraph::new(ctx);

    let lfo_freq = g.constant(0.5);
    let lfo_osc = g.sine(lfo_freq);
    let lfo_factor = g.constant(20.0);
    let lfo = g.mul(lfo_osc, lfo_factor);

    let base_freq = g.constant(440.0);
    let freq = g.add(lfo, base_freq);

    g.sine(freq);

    audio::run(&event_loop, g);
}
