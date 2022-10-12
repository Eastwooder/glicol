use crate::{Buffer, Input, Node, Message};
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Destination;

impl<const N: usize> Node<N> for Destination {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [[f32; N]]) {

        for i in 0..N {
            for j in 0..output.len() {
                output[j][i] = 0.0;
            }
        };

        // let in_num = inputs.len();

        for i in 0..N {
            // for j in 0..in_num {
            for (_key, value) in inputs.iter() {
                output[0][i] += value.buffers()[0][i];
                if value.buffers().len() >= 2 {
                    output[1][i] += value.buffers()[1][i];
                } else {
                    if output.len() >= 2 {
                        output[1][i] += value.buffers()[0][i];
                    }
                }
            }
        }
    }
    fn send_msg(&mut self, _info: Message) {
        
    }
}

// impl<const N: usize> Node<N> for SumBuffers {
//     fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
//         // Get the first output buffer.
//         let mut out_buffers = output.iter_mut();
//         let out_buffer_first = match out_buffers.next() {
//             None => return,
//             Some(buffer) => buffer,
//         };
//         // Fill it with silence.
//         out_buffer_first.silence();
//         // Sum all input buffers onto the first output buffer.
//         for input in inputs.values() {
//             for in_buffer in input.buffers() {
//                 dasp_slice::add_in_place(out_buffer_first, in_buffer);
//             }
//         }
//         // Write the first output buffer to the rest.
//         for out_buffer in out_buffers {
//             out_buffer.copy_from_slice(out_buffer_first);
//         }
//     }
//     fn send_msg(&mut self, _info: Message) {
        
//     }
// }
