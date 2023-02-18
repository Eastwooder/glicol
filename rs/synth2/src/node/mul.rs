use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Mul { val: f32, input_order: Vec<usize> }

impl Mul {
    pub fn new(val: f32) -> Self {
        Self { 
            val,
            input_order: vec![]
        }
    }
    impl_to_boxed_nodedata!();
    // pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
    //     NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    // }
}

impl<const N:usize> Node<N> for Mul {
    /// you can have many inputs as main or sidechain a, b, c...
    /// each input can have X channels: X can be 1, 2, 3, etc.
    /// we need to handle the case when `mul` output chan != input channels

    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs {:?} self.input_order {:?}", inputs, self.input_order);

        match inputs.len() { // to determine if there is a sidechain
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                for i in 0..output.len() {
                    for j in 0..N {
                        output[i][j] = main_input.buffers()[i][j] * self.val;
                    }
                }
            },
            2 => {
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                // println!("sidechain input node id for mul {}", ref_input.node_id);
                // println!("main input node id for mul {}", main_input.node_id);
                for i in 0..output.len() {
                    for j in 0..N {
                        output[i][j] = main_input.buffers()[i][j] * ref_input.buffers()[i][j];
                    }
                }
            },
            _ => {}
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {self.val = value},
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}