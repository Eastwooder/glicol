## Dev note

I start to rewrite this on Feb 17, 2023.
First get the constsig and mul to work and start to experiment on different channel matching here.

Do we need to limit such a situation: add a mono node inside a stereo context.

I think we should, although it means some extra expense on the performance.

## Todos
- [] provide some Error enums
- [] handle the channel issues better
- [] make it easier to specify the input
- [] write the test better (previously we rely on the plotting things; maybe we add `cpal` this time)