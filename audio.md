# Nightfire Audio

Audio processing code.  Contains structs to do biquad filtering.  Will
also contain code to load a neural network and do beat detection.


## TODO

### Audio normalization

Normalize the features in the samples by the max value.  The value
should be a floating max, maybe even with some outlier removal first.

The SignalProcessor contains the data to predict from.  It should be
in a Arc<Mutex<..>> which gets updated by the jack handler and gets
read by the beat detector.  Whenever a new sample has been generated
it takes the current history and predicts the beat for the sample that
is currently being read.
