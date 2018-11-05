//! # Sound Garden
//!
//! Sound Garden is a virtual modular audio synthesis environment.

extern crate audio_graph;
#[macro_use]
extern crate conrod;
extern crate cpal;
extern crate find_folder;
extern crate parking_lot;

mod audio;
mod prelude;
mod ui;

/// Right now code in the `main` is just a playground for testing API and UI design.
/// With time it should:
/// * Process command-line arguments to configure SG instance
///    (list/select audio backend/device, on/off input/OSC, OSC ports, path to record session etc.)
/// * Initialize OSC and run background OSC server thread
/// * Initialize audio and run background audio event loop thread
/// * Initialize UI and run its event loop
fn main() {
    let g = audio::init();

    {
        let mut g = g.lock();

        // A bit of a simple FM synth, WoooWoooWooo!
        //
        // We use "sweet" API which hides creating node, adding node and connecting node's inputs
        // behind a single AudioGraph method. All functions used below return node index. Index
        // could be used to reference node during entire program lifetime because AudioGraph is
        // backed by StableGraph. When maximum flexibility is needed, each step could be performed
        // separately, for example:
        //
        // let freq = Constant::new(&g.ctx, 440.0);
        // let freq = g.add_node(freq);
        // let saw = Phasor::new(&g.ctx);
        // let saw = g.add_node(saw);
        // g.connect(freq, saw);

        let lfo_freq = g.constant(0.5);
        let lfo_osc = g.sine(lfo_freq);
        let lfo_factor = g.constant(20.0);
        let lfo = g.mul(lfo_osc, lfo_factor);

        let base_freq = g.constant(440.0);
        let freq = g.add(lfo, base_freq);

        g.sine(freq);
    }

    ui::main();
}
