use fixedbitset::FixedBitSet;
use petgraph::visit::Topo;

use prelude::*;

pub type Node = Box<Module + Send>;

pub struct AudioGraph {
    pub ctx: Context,
    graph: StableGraph<Node, ()>,
    topo: Topo<NodeIndex, FixedBitSet>,
    input: Vec<Sample>,
    output: Vec<Sample>,
}

impl AudioGraph {
    pub fn new(ctx: Context) -> Self {
        let graph = StableGraph::default();
        let topo = Topo::new(&graph);
        let output = vec![0.0; ctx.channels];
        AudioGraph {
            ctx,
            graph,
            topo,
            input: vec![0.0; 256],
            output,
        }
    }

    pub fn sample(&mut self) -> &[Sample] {
        let input = &mut self.input;
        let ctx = &mut self.ctx;
        let channels = ctx.channels;
        let mut last_node = None;
        self.topo.reset(&self.graph);
        while let Some(idx) = self.topo.next(&self.graph) {
            last_node = Some(idx);
            let g = &mut self.graph;
            // NOTE it walks edges starting from the most recently added so we might want to make
            // one-by-one connect private and expose resetting all incoming connections at once as
            // an alternative we can use named edges and pass inputs as a hashmap but it's more expensive
            for (i, source) in g.neighbors_directed(idx, Incoming).enumerate() {
                let offset = i * channels;
                let output = g[source].output();
                input[offset..(channels + offset)].clone_from_slice(&output[..channels])
            }
            g[idx].sample(ctx, input);
        }
        if let Some(idx) = last_node {
            let g = &mut self.graph;
            let out = &g[idx].output();
            self.output[..ctx.channels].clone_from_slice(&out[..ctx.channels]);
        }
        ctx.sample_number += 1;
        &self.output
    }

    pub fn add_node(&mut self, n: Node) -> NodeIndex {
        self.graph.add_node(n)
    }

    pub fn connect(&mut self, a: NodeIndex, b: NodeIndex) {
        self.clear_inputs(b);
        self.graph.update_edge(a, b, ());
    }

    pub fn clear_inputs(&mut self, sink: NodeIndex) {
        while let Some(edge) = self
            .graph
            .neighbors_directed(sink, Incoming)
            .detach()
            .next_edge(&self.graph)
        {
            self.graph.remove_edge(edge);
        }
    }

    pub fn set_inputs(&mut self, sink: NodeIndex, sources: &[NodeIndex]) {
        self.clear_inputs(sink);
        for source in sources.iter().rev() {
            self.graph.update_edge(*source, sink, ());
        }
    }

    pub fn chain(&mut self, nodes: &[NodeIndex]) {
        for i in 0..(nodes.len() - 1) {
            self.clear_inputs(nodes[i + 1]);
            self.graph.update_edge(nodes[i], nodes[i + 1], ());
        }
    }

    pub fn output(&self) -> &[Sample] {
        &self.output
    }
}
