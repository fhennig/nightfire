# Mixxx data loader

The data is in `~/.mixxx/mixxxdb.sqlite`.  In the `beats` column there
is a protobuf structure which contains the bpm as a float and the
offset in integers for the beat onset.

## TODO

move the signal processor from `nightfire` into a seperate crate.


