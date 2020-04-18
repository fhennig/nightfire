# Mixxx data loader

The data is in `~/.mixxx/mixxxdb.sqlite`.  In the `beats` column there
is a protobuf structure which contains the bpm as a float and the
offset in integers for the beat onset.

## TODO

Next step is to check the infinite history in the signal_processor.
Run it and read it and check if it works.

Also, add a check to the hist_len in the SignalProcessor.  A length of
0 is not possible.

