use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
use arrayvec::ArrayVec;

#[derive(Debug, Clone)]
pub struct ConstSig {
    val: f32,
    events: ArrayVec<(f32, f32), 128>,
    pattern: ArrayVec<(f32, f32), 128>,
    span: f32,
    bpm: f32,
    sr: usize,
    step: usize,
    input_order: ArrayVec<usize, 64>,
}

impl ConstSig {
    pub fn new(val: f32) -> Self {
        Self {
            val,
            events: ArrayVec::<_, 128>::new(),
            pattern: ArrayVec::<_, 128>::new(),
            span: 1.,
            bpm: 120.,
            sr: 44100,
            step: 0,
            input_order: ArrayVec::<usize, 64>::new()
        }
    }

    pub fn events(self, events: ArrayVec<(f32, f32), 128>) -> Self {
        Self {events, ..self}
    }

    pub fn pattern(self, pattern: ArrayVec<(f32, f32), 128>) -> Self {
        Self {pattern, ..self}
    }

    pub fn span(self, span: f32) -> Self {
        Self {span, ..self}
    }

    pub fn bpm(self, bpm: f32) -> Self {
        Self {bpm, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    impl_to_boxed_nodedata!();
}

impl<const N:usize> Node<N> for ConstSig {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [[f32; N]]) {
        
        let cycle_dur = 60. / self.bpm * 4.;
        let bar_dur = cycle_dur * self.span * self.sr as f32;

        for i in 0..N {

            for event in &self.events {
                if (self.step % (bar_dur as usize)) == ((event.1 * cycle_dur * self.sr as f32) as usize) {
                    self.val = event.0
                }
            }

            for event in &self.pattern {
                if (self.step % (bar_dur as usize)) == ((event.1 * cycle_dur * self.sr as f32) as usize) {
                    self.val = event.0
                }
            }
            output[0][i] = self.val;
            self.step += 1;
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetPattern(p, span) => {
                self.pattern = p;
                self.span = span;
            },
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
            _ => {}
        }
    }
}