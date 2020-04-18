# Mixxx data loader

The data is in `~/.mixxx/mixxxdb.sqlite`.  In the `beats` column there
is a protobuf structure which contains the bpm as a float and the
offset in integers for the beat onset.

## TODO

Serialize into pickle; the npy crate does not support ndim > 1.
there is serde-pickle for rust, that should work better.
