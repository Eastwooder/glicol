/// channes can be problematic sometimes
/// let's say you have a stereo sample
/// then it passed through a lowpass filter
/// then the lowpass filter have to know that it's a stereo work to do
/// and in the future, the lpf should also prepare for multi-channel processing

use glicol_synth2::{
    AudioContextBuilder,
    ConstSig,
    Message
};

fn main() {
    let mut context = AudioContextBuilder::<4>::new()
    .sr(44100)
    .channels(2)
    .build();

    let node_a = context.add_mono_node(ConstSig::new(42.));
    context.connect(node_a, context.destination);
    println!("first block {:?}", context.next_block());
    context.send_msg(node_a, Message::SetToNumber(0, 100.) );
}