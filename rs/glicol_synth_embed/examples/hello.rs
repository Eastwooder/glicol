use glicol_synth_embed::{AudioContextBuilder, ConstSig, Message};
use libc_print::libc_println;

fn main() {
    let mut context = AudioContextBuilder::<16>::new()
    .sr(44100).channels(1).build();

    let node_a = context.add_mono_node(ConstSig::new(42.));
    context.connect(node_a, context.destination);
    libc_println!("first block {:?}", context.next_block());

    context.send_msg(node_a, Message::SetToNumber(0, 100.) );
    libc_println!("second block, after msg {:?}", context.next_block());

}