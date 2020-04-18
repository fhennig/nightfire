# Python Audio Analysis and ML Training

With the data generated from the `mixxx_data` crate I want to train a
neural network to predict beats in real time.  The data is generated
by the rust crate, using the same feature extraction as it would be
used later in the live processing.

The generated data with annotated beats from Mixxx can then be used to
train a model.  This training will be done in python.

## TODO

The data out of the rust code is a long matrix T x D, with T time
steps and D feature dimensions. The target vector is of length T, with
a bool value at every step if there is a beat or not.
