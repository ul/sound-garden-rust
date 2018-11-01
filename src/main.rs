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

use prelude::*;

fn main() {
    let (event_loop, ctx) = audio::init();
    let mut graph = AudioGraph::new();

    let lfo_freq = graph.add_node(Constant::new(&ctx, 0.5));
    let lfo_phase = graph.add_node(Phasor::new(&ctx));
    let lfo_osc = graph.add_node(Fn1::new(&ctx, sine));
    let lfo_factor = graph.add_node(Constant::new(&ctx, 20.0));
    let lfo = graph.add_node(Fn2::new(&ctx, mul));

    let base_freq = graph.add_node(Constant::new(&ctx, 440.0));
    let freq = graph.add_node(Fn2::new(&ctx, add));
    let phase = graph.add_node(Phasor::new(&ctx));
    let osc = graph.add_node(Fn1::new(&ctx, sine));

    graph.connect(lfo_freq, lfo_phase);
    graph.connect(lfo_phase, lfo_osc);
    graph.connect(lfo_osc, lfo);
    graph.connect(lfo_factor, lfo);
    graph.connect(base_freq, freq);
    graph.connect(lfo, freq);
    graph.connect(freq, phase);
    graph.connect(phase, osc);

    audio::run(&event_loop, ctx, graph);
}
