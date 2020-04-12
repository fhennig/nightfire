# Beat Detection with ML

I had an idea for a machine learning project to create a network that
can detect beat and BPM live, online, in a very performant way.

## What does "beat" and BPM mean?

Detection is performed on a slice of audio.  The slice is on beat if
the slice *ends* at the start/end of a beat.  This start/end is not
necessarily well defined, but can be defined as a region of maybe 20ms
in which the onset of the beat happens.  This often coincides with the
hit of kick drum or a snare.  The onset of the hit of the drum can be
seen in the frequency spectrum with a clicking.  This is not in a
single sample, but in a time slice.

The bpm is the frequency at which the beats occur.  In theory this
could be any floating point number, but music is typically made with a
bpm that is an integer, such as 128, 140 or 175.  There is also a
range of BPMs that are common, and this might depend on the use case.
I'd say that a range of 60 to 200 covers most music.  This gives 140
different speeds, which are the class labels.

### How accurate can a human tap on the beat?

A quick test showed me that at a beat of 128 BPM I could tap along
with a standard deviation of about 15 to 20ms.  Assuming the taps are
approximately following a normal distribution, that would mean that
about 70% of my taps were in a 40ms window around the "actual"
position of the beat.

For a song with 173 BPM I got a sigma of around 10ms.  With 110 BPM I
got around 35ms.

This suggests that the accuracy also depends on the speed.  If we
divide the twice the deviation -- too early or too late -- by the BPM
to get the duration of the beat as perceived by a human we get around
5% to 10% of a beat period.

5.7% of a 173 BPM beat is still 20ms, which means it is a good
estimate.  20ms is 4.3% of a 128 BPM beat.

This tells me that testing every 20ms (=> at 50Hz) will give a good
accuracy.

It also means the model will see only 5% positive samples and 95%
negative samples.

...

However, if I do this subsampling at 50Hz, the difference between 128
and 129 does not show within 4 beats.  It needs more time for them to
deviate, like 16 beats.  With 32 beats that would be 15 seconds of
audio.

## Model

The model takes a time slice and predicts whether there is currently a
beat or not.  If there is a beat, it also determines the periodicity
of the beat (the BPM).  To find the onset of the beat the model needs
to slide over the signal, triggering when the sliding window is just
over the onset of the beat.

We say that a beat happens in a 20ms window.  The input window to the
model this slides over the signal at 20ms intervals, which gives a
frequency of evaluating the model 50 times a second.  The model is
designed such that this high frequency is achievable, due to the model
being very compact.

### Input and feature extraction

The input to the model is a slice of audio which consists of samples
with a sampling frequency f_S.  This is the frequency at which the
analog signal is sampled, typical values are 44100Hz and 48kHz.  The
slice should be a few seconds long, let the length in seconds be L.

A signal can be interpreted as a sum of different individual signals
at different frequencies, corresponding to the different instruments
in the music.  The audible spectrum of humans ranges from 20Hz to
22kHz.  We can define ranges such as the bass range from 60Hz to
250Hz.

We take k evenly spaced frequencies which we will measure as
frequencies (f_1 to f_k).  In the process of dividing the signal in
this way we also subsample in the time domain, accumulating a signal
of a few ms, such as 10ms into a single sample of k features.  Lets
call call this subsampling frequency f_T.

Now if we take a 4 second input sample with 100Hz subsampling and 50
features, we get 20,000 values.  With just 50Hz and 20 features it
would be 4,000 values.  This is the dimensionality of our input.

The features can be extracted with computationally cheap infinite
impulse response filters, also called biquad filters.  A single
bandpass filter can be computed in O(n) where n is the length of the
sample.  The features would be k such bandpass filters with
frequencies f_1 to f_k.  They would be evenly spaced in log10 space,
because we hear frequency differences in log space too.

### The network

The input has now been converted from a long, high-frequent sequence
of numbers to a less frequent sequence of feature vectors. The sample
is now T x K with T being the time steps and K the amount of features.

We can now imagine having a slice of audio for which we want to detect
whether it is playing at 128 BPM and where the beat starts.  We can
design a kernel that would slide over the audio and detect these
periodic beats as it slides over the signal.  We can then also design
further kernels for other speeds, one kernel per speed (class).  an
extra kernel can be used to detect unrythmic signals, noise and
silence.

Instead of having a fixed slice and convolving over it, we convolve
over the signal as it is generated live.  So instead of having a
convolution in the network, the network is just evaluated with
different inputs generated from a sliding window.

To detect the periodicity we need a slice of audio that is long enough
to capture a few repetitions of the beat. To get 4 beats in a 60 BPM
signal we need 4 seconds of signal.  If the periodicity is encoded in
the whole kernel, we would need a kernel that looks at a 4 second
window.

All the kernels are multiplied with the input and then we apply a
softmax to get the classification.

Designed in this way the model would output the "NOISE" class whenever
there is not currently a beat, and if a beat is happening it would
output "BEAT-128" or "BEAT-125" or similar, at the time slice where
the beat is just starting.

The input and kernels can be flattened, and then the model would be a
simple single layer feed-forward network.  The input dimensionality is
determined by k, f_T, and L, and the output dimensionality by the
number of classes (C).  The number of weights is in the range of 1M.

### Kernel Design

Given the setup of the problem it seems plausible that not every
kernel would need to look at all frequencies at the frequency f_T.
Potentially the beat can be detected mostly in the bass frequency
range or in the brilliance above 6kHz, where hi-hats and claps can be
found.  The rest of the weights could be set to zero.  L1
regularization could force the model to pick only a few features to
decide on.

Kernels could also have individual subsampling frequencies that
correspond to the BPM that they are supposed to detect.

### Computational demand

This design is made specifically for live processing on a CPU also on
small devices such as a Raspberry Pi.  The kernels could easily be
parallelized on multiple CPU cores.  In a CPU implementation using
O(1) array access and no matrix multiplication optimization, the
sparsity of the weight matrices would actually result in faster
computations, since those array accesses and multiplications can be
skipped.

At every time slice the features need to be extracted and then the
kernels need to be multiplied and summed, and then a softmax needs to
be applied on a C-dimensional vector.  All in all this is
computationally very quick.

To reduce the load, the classes that are checked can be changed at
runtime.  Maybe for an hour only songs in a range from 120 to 140 bpm
will be played.  Then suddenly the number of classes is reduced from
140 to just 20, this speeds up computation a lot.

## Data

The input is slices of audio for which bpm and and beat offset is
known.

The training data can be created from a few songs for which bpm and
offset is known.  From this, samples can be generated by taking slices
out of the audio, some of which will be on beat, some of them will be
off beat.

### Data Augmentation

From a single song or audio sample file, data can be augmented.  Noise
can be added to the sample, either overall, or just at specific
frequencies.  Also the overall sample can be sped up or slowed down
(within some limits, i.e. +-10BPM).

Speeding up or slowing down would have to happen first, resampling can
be implemented by linearly interpolating between two existing samples.
Feature extraction should happen afterwards, with noise applied to the
features.

Noise can be applied in specific frequency bands such as the mids, to
make the model rely less on the mids where the melodies and voices
are.

## Implementation

Some notes on implementation.  Current experimentation has been
conducted in Rust, because its very fast and well suited for live
processing because of that.  The final use of the model would also be
in rust.

For training of the model, speed is not essential, instead if might be
good to benefit from the rich ecosystem for machine learning in
python.

As of now I had a quick look and saw the that ML ecosystem in rust is
not very mature yet.  But for python it is also not so easy to work
with audio, and the feature extraction system has already been
implemented in rust.

Maybe a combined approach can work, where data is loaded, augmented
and feature extraction is performed, and the generated data is then
written back to files.  The training files are then used in a python
learning module.  The learnt modules are saved such that they can be
loaded in rust again for production use.

### Data

I can get training data from [Mixxx](https://www.mixxx.org/), where I
already "annotated" a lot of songs with the correct BPM and beat
offset.

The information is stored in an sqlite file at
`~/.mixxx/mixxxdb.sqlite`.  The songs are stored in a table called
`library` and the information is stored in the column `beats`.  The
format is a `protobuf` serialized structure.  Deserialized it contains
two fields, a 64-bit float and an integer.  The float is the BPM and
the integer is the frame offset of the first beat.

## Another Idea: Beat first, speed after

I realized that if only a 4 second slice is taken, 128 and 129 BPM
will be indistinguishable.  A beat is 468.75ms and 465.12ms
respectively, which, if divided by 20ms and rounded is both a length
of 23 20ms subsamples.  A kernel looking for beats with a length of 24
samples detects 125 BPM perfectly, with 23 samples it detects 130.4
BPM.

With 30 kernels detecting lengths from 45 to 15 subsamples, beats from
68 to 200 BPM can be detected.  If each kernel has just the length of
one beat, with the kernel for 200 BPM gets 15 samples, the one for 68
BPM gets 45.  If each sample is 30 features wide, with the 30 kernels
thats just 26,550 weights.

This could be efficiently calculated!  It could even be parallelized
quite a bit.

With this, I could attach a label to the the samples, and if there are
intertwined labels of 125 and 130, the actual BPM can be reconstructed
from that over time.

It doesn't need to be entirely correct because the algorithm would
correct itself over time.
