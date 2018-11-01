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
    let mut graph = AudioGraph::new();

    let lfo_freq = graph.add_node(ctx.constant(0.5));
    let lfo_phase = graph.add_node(ctx.phasor());
    let lfo_osc = graph.add_node(ctx.sine());
    let lfo_factor = graph.add_node(ctx.constant(20.0));
    let lfo = graph.add_node(ctx.mul());

    let base_freq = graph.add_node(ctx.constant(440.0));
    let freq = graph.add_node(ctx.add());
    let phase = graph.add_node(ctx.phasor());
    let osc = graph.add_node(ctx.sine());

    graph.chain(&[lfo_freq, lfo_phase, lfo_osc]);
    graph.set_inputs(lfo, &[lfo_osc, lfo_factor]);
    graph.set_inputs(freq, &[base_freq, lfo]);
    graph.chain(&[freq, phase, osc]);

    audio::run(&event_loop, ctx, graph);
}
