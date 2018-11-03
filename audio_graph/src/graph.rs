//! # Audio graph
use fixedbitset::FixedBitSet;
use petgraph::visit::Topo;

use prelude::*;

pub type Node = Box<Module + Send>;

/// Structure which manages network of Modules
pub struct AudioGraph {
    /// audio context (channel count, sample rate, time etc.)
    pub ctx: Context,
    /// nodes are boxed Modules and edges represent source->sink connections
    graph: StableGraph<Node, ()>,
    /// `sample` walks graph in topological order; Topo structure is stored in AudioGraph to avoid
    /// memory allocations in `sample`, which just resets it before traversal
    topo: Topo<NodeIndex, FixedBitSet>,
    /// `sample` writes output from source nodes into this buffer and then passes it to sink
    /// buffer is reused during graph traversal and between samples to avoid memory allocations
    input: Vec<Sample>,
}

/// Maximum number of sources to connect to sink.
/// This number is required because input buffer is allocated during AudioGraph initialization and
/// then re-used across all nodes sampling. There is no real need to have it hardcoded though.
/// It can be made an argument of AudioGraph::new
const MAX_SOURCES: usize = 16;

impl AudioGraph {
    pub fn new(ctx: Context) -> Self {
        let graph = StableGraph::default();
        let topo = Topo::new(&graph);
        let input_len = ctx.channels() * MAX_SOURCES;
        AudioGraph {
            ctx,
            graph,
            topo,
            input: vec![0.0; input_len],
        }
    }

    /// Compute and return the next frame of AudioGraph's sound stream.
    /// Frame slice contains one sample for each channel.
    pub fn sample(&mut self) -> &Frame {
        let mut last_node = None;
        {
            let input = &mut self.input;
            let ctx = &mut self.ctx;
            let channels = ctx.channels();
            self.topo.reset(&self.graph);
            while let Some(idx) = self.topo.next(&self.graph) {
                last_node = Some(idx);
                let g = &mut self.graph;
                // NOTE neighbors_directed walks edges starting from the most recently added (is it
                // guaranteed?). This is the reason why connection methods (connect, set_sources,
                // chain etc.) call clear_sources first and reverse sources. Always resetting
                // sources instead of finer-grained manipulation reduces risk of confusing their
                // order. We might want to consider to name edges and pass HashMap instead instead
                // of Vec as input. But it implies non-neglegible performance hit.
                //
                // ref `Module::sample` doc for an example of input layout
                for (i, source) in g.neighbors_directed(idx, Incoming).enumerate() {
                    let offset = i * channels;
                    let output = g[source].output();
                    input[offset..(offset + channels)].clone_from_slice(&output[..channels])
                }
                g[idx].sample(ctx, input);
            }
            ctx.tick();
        }
        if let Some(idx) = last_node {
            &self.graph[idx].output()
        } else {
            // This might be a garbage, we just don't care what to return from empty graph
            // and don't want to allocate any extra resources for such case.
            &self.input
        }
    }

    /// Add node to the graph and return index assigned to the node.
    /// This index is stable and could be used to reference the node when building connections.
    pub fn add_node(&mut self, n: Node) -> NodeIndex {
        self.graph.add_node(n)
    }

    /// Connect nodes in a chain, from left to right.
    /// It clears nodes' sources before connecting, except for the first one.
    pub fn chain(&mut self, nodes: &[NodeIndex]) {
        for i in 0..(nodes.len() - 1) {
            self.clear_sources(nodes[i + 1]);
            self.graph.update_edge(nodes[i], nodes[i + 1], ());
        }
    }

    /// Set node `a` as a single source of node `b`.
    /// It clears `b`'s sources before connecting, to set multiple sources use `set_sources`.
    pub fn connect(&mut self, a: NodeIndex, b: NodeIndex) {
        self.clear_sources(b);
        self.graph.update_edge(a, b, ());
    }

    /// Set multiple sources for the `sink` node.
    /// It clears `sink`'s sources before connecting.
    /// `source`s' outputs are layouted in `sink` input buffer in the provided order.
    /// Ref `Module::sample` doc for an example of input layout.
    pub fn set_sources(&mut self, sink: NodeIndex, sources: &[NodeIndex]) {
        self.clear_sources(sink);
        // ref `sample` method comments for the reason of reversing sources
        for source in sources.iter().rev() {
            self.graph.update_edge(*source, sink, ());
        }
    }

    /// Remove all incoming connections of the node.
    fn clear_sources(&mut self, sink: NodeIndex) {
        while let Some(edge) = self
            .graph
            .neighbors_directed(sink, Incoming)
            .detach()
            .next_edge(&self.graph)
        {
            self.graph.remove_edge(edge);
        }
    }
}
