#![cfg_attr(not(feature = "std"), no_std)]

// #[no_mangle]

// use alloc::ArrayVec;

// mod context;
// pub use context::*;

// mod graph;
// pub use graph::*;

mod node;
pub use node::{Input, Node};

// pub use node::{
//     oscillator, 
//     filter, 
//     effect, 
//     envelope, 
//     operator, 
//     sequencer, 
//     signal,
//     delay,
//     compound,
//     synth
// };

// pub use node::*; // TODO: Do not expose every struct here

mod buffer;
pub use buffer::Buffer;

// #[cfg(feature = "node-sampling")]
// pub use node::{sampling};

// #[cfg(feature = "node-dynamic")]
// pub use node::{dynamic};

#[cfg(feature = "node-boxed")]
pub use node::{BoxedNode, BoxedNodeSend};

// #[cfg(feature = "node-sum")]
// pub use node::{Sum, Sum2};

#[cfg(feature = "node-pass")]
pub use node::{Pass};

use hashbrown::HashMap;
// pub use hashbrown::HashMap;
pub use arrayvec::{ArrayVec, ArrayString};

// #[macro_export]
// macro_rules! impl_to_boxed_nodedata {
//     () => {
//         pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
//             NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
//         }
//     };
// }

#[derive(Debug, Clone)]
pub enum Message {
    SetToNumber(u8, f32),
    SetToNumberList(u8, ArrayVec<f32, 32>),
    SetToSymbol(u8, ArrayString<32>),
    // SetToSamples(u8, (&'static [f32], usize, usize)),
    SetSamplePattern(ArrayVec<(ArrayString<32>, f32), 32>, f32, HashMap<ArrayString<32>, (&'static [f32], usize, usize)>),
    // SetPattern(ArrayVec<(f32, f32)>, f32),
    // SetToSeq(u8, ArrayVec::<(f32, GlicolPara)>),
    // SetRefOrder(HashMap<ArrayString, usize>),
    // SetBPM(f32),
    // SetSampleRate(usize),
    // MainInput(petgraph::graph::NodeIndex),
    SidechainInput(petgraph::graph::NodeIndex),
    Index(usize),
    IndexOrder(usize, usize),
    ResetOrder
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum GlicolPara {
    Number(f32),
    // NumberList(ArrayVec<f32>),
    // Reference(ArrayString),
    // SampleSymbol(ArrayString), // symbol is for sample only
    // Symbol(ArrayString),
    // Sequence(ArrayVec::<(f32, GlicolPara)>),
    // Pattern(ArrayVec::<(GlicolPara, f32)>, f32),
    // Event(ArrayVec::<(GlicolPara, f32)>)
}