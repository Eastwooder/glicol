use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
// use rustfft::FftPlanner;
// use rustfft::num_complex::Complex;
// use rustfft::num_traits::Zero;
// use std::f32::consts::PI;
use sonic_rs;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
// use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sonic {
    stream: ManuallyDrop<sonic_rs::Stream>,
    phantom: PhantomData<sonic_rs::Stream>,
    speed: f32,
    pitch: f32,
    rate: f32,
    volumn: f32,
    input_order: Vec<usize>
}

unsafe impl Send for Sonic {}

impl Drop for Sonic {
    fn drop(&mut self) {
        unsafe {
            sonic_rs::destroy_stream(*self.stream);
            ManuallyDrop::drop(&mut self.stream);
        }
    }
}

impl Sonic {
    pub fn new(speed: f32, pitch: f32, rate: f32, volumn: f32, sr: usize) -> Self {
        let stream =  ManuallyDrop::new(sonic_rs::create_stream(sr as u32, 1)); // hard-coded 1 chan...
        sonic_rs::sonic_set_pitch(*stream, pitch);
        sonic_rs::sonic_set_speed(*stream, speed);
        
        Self {
            stream,
            phantom: PhantomData,
            speed, pitch, rate, volumn,
            input_order: vec![]
        }
    }

    // fn stream_ptr(&self) -> sonic_rs::Stream {
    //     *self.stream
    // }

    impl_to_boxed_nodedata!();

}

impl<const N:usize> Node<N> for Sonic {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let inbuf = &inputs.values_mut().next().unwrap().buffers();
        // panic!();
        sonic_rs::write_float_to_stream(*self.stream, inbuf[0].as_ptr(), N as u16);
        sonic_rs::read_float_from_stream(*self.stream, output[0].as_mut_ptr(), N as u16);
        // for chan in 0..inbuf.len() {
        //     for i in 0..N {
        //         output[chan][i] = inbuf[chan][i]
        //     }
        // }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, _v) => {
                match pos {
                    // 0 => {self.pitch_shift = value},
                    // 1 => {self.time_stretch = value},
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