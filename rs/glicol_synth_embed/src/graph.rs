use crate::{
    buffer::Buffer,
    node::Input,
    node::Node,
    BoxedNode,
};

use hashbrown::HashMap;
use arrayvec::ArrayVec;

extern crate alloc;
use alloc::boxed::Box;

use petgraph::data::{DataMap, DataMapMut};
use petgraph::visit::{
    Data, DfsPostOrder, GraphBase, IntoNeighborsDirected, Reversed, //NodeCount, NodeIndexable, 
    Visitable,
};

use petgraph::{Incoming};

pub struct Processor<G, const N: usize>
where
    G: Visitable,
{
    // State related to the traversal of the audio graph starting from the output node.
    dfs_post_order: DfsPostOrder<G::NodeId, G::Map>,
    // Solely for collecting the inputs of a node in order to apply its `Node::process` method.
    inputs: HashMap<usize, Input<N>>,
    // pub processed: Vec<G::NodeId>
}

/// For use as the node weight within a dasp graph. Contains the node and its buffers.
///
/// For a graph to be compatible with a graph **Processor**, its node weights must be of type
/// `NodeData<T>`, where `T` is some type that implements the `Node` trait.
pub struct NodeData<T: ?Sized, const N: usize> {
    pub buffers: [f32; 2],
    pub node: T,
}

impl<G, const N: usize> Processor<G, N>
where
    G: Visitable + petgraph::visit::NodeIndexable,
{
    // TODO: remove this with_capacity; max_nodes: usize
    pub fn new() -> Self
    where
        G::Map: Default,
    {
        let mut dfs_post_order = DfsPostOrder::default();
        dfs_post_order.stack = ArrayVec::<_, 1024>::new().to_vec();
        let inputs = HashMap::new(); //Vec::with_capacity(max_nodes);
        Self {
            dfs_post_order,
            inputs,
        }
    }
    pub fn process<T>(&mut self, graph: &mut G, node: G::NodeId)
    where
        G: Data<NodeWeight = NodeData<T, N>> + DataMapMut,
        for<'a> &'a G: GraphBase<NodeId = G::NodeId> + IntoNeighborsDirected,
        T: Node<N>,
    {
        process(self, graph, node)
    }
}

impl<T, const N: usize> NodeData<T, N> {
    /// Construct a new **NodeData** from an instance of its node type and buffers.
    pub fn new(
        node: T, 
        buffers: [f32; 2]
    ) -> Self {
        NodeData { node, buffers }
    }

    // /// Creates a new **NodeData** with a single buffer.
    // pub fn new1(node: T) -> Self {
    //     // let mut vec = ArrayVec::<Buffer<N>, 8>::new();
    //     // vec.push(Buffer::SILENT);
    //     Self::new(node, [Buffer::SILENT; 1])
    // }

    // /// Creates a new **NodeData** with two buffers.
    // pub fn new2(node: T) -> Self {
    //     // let mut vec = ArrayVec::<Buffer<N>, 8>::new();
    //     // vec.push(Buffer::SILENT);
    //     Self::new(node, [Buffer::SILENT; 2])
    // }

    /// Creates a new **NodeData** with 8 buffers.
    pub fn multi_chan_node(chan: usize, node: T) -> Self {
        // let mut vec = ArrayVec::<Buffer<N>, 8>::new();
        // for _ in 0..chan {
        //     vec.push(Buffer::SILENT);
        // };
        Self::new(node, [0.0; 2])
    }
}

#[cfg(feature = "node-boxed")]
impl<const N: usize> NodeData<BoxedNode<N>, N> {
    /// The same as **new**, but boxes the given node data before storing it.
    pub fn boxed<T>(node: T, buffers: [f32; 2]) -> Self
    where
        T: 'static + Node<N>,
    {
        NodeData::new(BoxedNode(Box::new(node)), buffers)
    }

    // /// The same as **new1**, but boxes the given node data before storing it.
    // pub fn boxed1<T>(node: T) -> Self
    // where
    //     T: 'static + Node<N>,
    // {
    //     let mut vec = ArrayVec::<Buffer<N>, 8>::new();
    //     vec.push(Buffer::SILENT);
    //     Self::boxed(node, vec)
    // }

    // /// The same as **new2**, but boxes the given node data before storing it.
    // pub fn boxed2<T>(node: T) -> Self
    // where
    //     T: 'static + Node<N>,
    // {
    //     let mut vec = ArrayVec::<Buffer<N>, 8>::new();
    //     vec.push(Buffer::SILENT);
    //     vec.push(Buffer::SILENT);
    //     Self::boxed(node, vec)
    // }
}

pub fn process<G, T, const N: usize>(
    processor: &mut Processor<G, N>,
    graph: &mut G,
    node: G::NodeId,
) where
    G: Data<NodeWeight = NodeData<T, N>> + DataMapMut + Visitable + petgraph::visit::NodeIndexable,
    for<'a> &'a G: GraphBase<NodeId = G::NodeId> + IntoNeighborsDirected,
    T: Node<N>,
{
    const NO_NODE: &str = "no node exists for the given index";
    processor.dfs_post_order.reset(Reversed(&*graph));
    processor.dfs_post_order.move_to(node);
    while let Some(n) = processor.dfs_post_order.next(Reversed(&*graph)) {
        let data: *mut NodeData<T, N> = graph.node_weight_mut(n).expect(NO_NODE) as *mut _;
        processor.inputs.clear();
        for in_n in (&*graph).neighbors_directed(n, Incoming) {
            // Skip edges that connect the node to itself to avoid aliasing `node`.
            if n == in_n {
                continue;
            }
            // println!("{:?}", (&*graph).to_index(in_n));

            let input_container = graph.node_weight(in_n).expect(NO_NODE);
            let input = Input::new(&input_container.buffers, (&*graph).to_index(in_n));
            processor.inputs.insert((&*graph).to_index(in_n), input);
        }
        // Here we deference our raw pointer to the `NodeData`. The only references to the graph at
        // this point in time are the input references and the node itself. We know that the input
        // references do not alias our node's mutable reference as we explicitly check for it while
        // looping through the inputs above.
        unsafe {
            (*data)
                .node
                .process(&mut processor.inputs, &mut (*data).buffers);
        }
    }
}