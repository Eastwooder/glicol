use glicol_synth2::{
    AudioContextBuilder,
    GlicolSynthError,
    ConstSig,
    Mul,
    Message
};

fn main() -> Result<(), GlicolSynthError> {
    let mut context = AudioContextBuilder::<4>::new()
    .sr(44100)
    .channels(2)
    .build();

    // it will return an error to add a non-stereo node in a stereo context
    let node_a = context.add_stereo_node(ConstSig::new(42.))?;
    let node_b = context.add_stereo_node(Mul::new(0.5))?;
    context.connect(node_a, node_b);
    context.connect(node_b, context.destination);
    println!("first block {:?}", context.next_block());
    context.send_msg(node_a, Message::SetToNumber(0, 100.) );
    println!("2nd block {:?}", context.next_block());
    Ok(())
}