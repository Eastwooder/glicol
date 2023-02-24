## Dev note

I start to rewrite this on Feb 17, 2023.
First get the constsig and mul to work and start to experiment on different channel matching here.

Do we need to limit such a situation: add a mono node inside a stereo context.

I think we should, although it means some extra expense on the performance.

## Todos
- [x] provide some Error enums
- [x] handle the channel issues better
- [] make it easier to specify the input
- [] write the test better (previously we rely on the plotting things; maybe we add `cpal` this time)


## Thoughts

#### 20230218
A better graph constructor is needed.

```rust

fn main() -> Result<(), GlicolSynthError> {

    let mut graph = GlicolGraph::new();
    graph.insert("o", vec![GlicolNode::SinOsc(vec![440.0])]);

    let mut context = GlicolContextBuilder::<4>::new()
    .sr(44100)
    .channels(2)
    .set_graph(graph)
    .build();

    println!("first block {:?}", context.next_block());
    context.send_msg(node_a, GlicolMessage::SetToNumber(0, 100.) );
    println!("2nd block {:?}", context.next_block());
    Ok(())
}

```