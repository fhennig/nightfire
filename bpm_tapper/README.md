# BPM Tapper

A little program that reads irregular beat taps and calculates the
mean BPM and estimates the actual location of the beats.

## TODO

- Make a UI that shows the beat progression.

### Neural Network Tap Correction

The taps are potentially not exactly on the beat, since they are made
by a human and there is also input latency.  We can use a neural
network to detect the actual beat withing a small window of the manual
tap.

Make a beat detector network, binary classifier.  Should take ~ 15
frames with the beat expected on 5.  Returns whether there is a beat
there or not.

With a tapped beat location, generate a window where we think the beat
actually is.  For this window, convolve the binary classifier over it.
Assign the beat to the sample in the window that has the highest
probability of the beat being there.

In the actual code I would generate the audio samples.  Then, if a tap
is made, annotate this *manual* tap on the current sample.  To do the
prediction, it has to be waited to allow capturing the whole window.
After the window has been captured, run the prediction.  The resulting
corrected beat location can then be put into the beat tapper.

