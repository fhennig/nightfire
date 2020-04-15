# Ideas

Ideas and TODOs for the future.

## New Mode

A new mode where both sticks are a "blob" that is moved around.  One
is red, the other is blue.  Can be a nice mix of colors.  Make the
controller very symmetrical, have one side for each blob.

I could have both blobs be controlled by different intensities, one
for bass, one for high hats or something.

## Light intensity normalization function [medium]

I need a function that takes a linear 0 to 1 value and somehow maps it
to values that look linear in the lights too.  The lights are very
sensitive from 0 to about 0.15 and not so much above that.

## pi-blaster frequency [minor]

On the [piblaster-github](https://github.com/sarfata/pi-blaster) there
is a frequency about "How to adjust the frequency and the resolution
of the PWM", which I should look into more.  I can configure the
update frequency, which I have internally set to 50Hz.  I can also
define the resolution, which I did not consider so far.  By default it
is 1000.  I think I can reduce both of these numbers, maybe I can get
the performance up a little bit more.

## New workspace for loading samples from Mixxx

With sqlite and protobuf, load audio files and create numpy arrays for
training of a beat-detector.
