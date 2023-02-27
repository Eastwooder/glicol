# Glicol Parser

The parser converts string input to an AST.
Later we can use the AST to construct the audio graph in the main crate.

```
t1: sin 440 >> mul 0.1
```

```
t1: sin {freq: ~mod, phase: 0.5} >> mul 0.1
```

```
t1: seq 60 _60 {span: 2.0, bpm: 120.0} >> sampler \jazz_song {start: 0.1, bpm: 132, pitch: +12}
```

```
t1: seq {pattern: "60 _60 _ 60", span: 2.0} >> sampler {name: \jazz_song, }
```

```
t1: seq 60 _60 {speed: 2.0, phase: 3.0} >> sampler \jazz_song {start: 0.1, bpm: 132, pitch: +12}
```

```
~env: imp 1.0 >> envperc {attack: 0.1, decay: 0.2}
```

So, a node always start from some `obvious` or `intuitive` parameters.

```
~seq: seq 60 _60 _ 60 {span: 1.0}

t1: ~seq >> sampler \dude {
  start: 2100_ms,
  end: 2100_ms + 100_ms,
  trigger: 60,
  attack: 30_ms,
  decay: 200_ms,
  pitch: -12,
}
```