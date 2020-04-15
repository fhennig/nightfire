# lichtspiel

A headless, drop-in audio to light visualizer build for the raspberry
pi.

- Input raw audio from spotify, an aux-in or something similar.
- Processing (filtering, reverb etc).
- Visualization on LEDs connected to GPIO.

Interfaces with JACK audio, and can take any audio stream.  It's
suggested to use Carla with it to do some audio processing, and pipe
it into this software afterwards.

## Jack, Carla, Midi

Currently the project registers itself as a jack client and connects
to an audio stream, which it processes and then displays in a variety
of ways.

### Modularizing

There are no modules at the moment and it isn't easy to integrate
other software.  Also the different output modules are all compiled
into the binary.  Internally there is some multithreading going on,
this could all be externalized by letting a plugin host or server
handle the interfacing of modules.

MIDI could be a way to achive this.  Modules could communicate with
MIDI signals.  I could have a module that converts audio to midi and
a few more for different visualizations of that midi.

In MIDI there are cc events which have values from 0 to 127, which can
encode a float.  These are perfect to encode my light intensity
values.  pi-blaster only uses a resolution of 100 by default; so even
though a number is typically 0 to 255 this should not be an issue.

### Carla

At the moment, interfacing is only with Jack.  Carla is a VST host
that can also be run headless and integrates well with Jack.  I can
some tests with it an it has the benefit of enabling the use of
VST/LV2/LAPSDA plugins.  This means for audio processing, standard
tools can be used.

Carla can be compiled for ARM, but many plugins do not provide an ARM
binary, so this path might be hard to take.

## Raspberry Pi Interfacing

lichtspiel is supposed to work on a Raspberry Pi.

### Cross-Compiling Rust

In essence, install the target with rustup, install the armhf linker
and any additional dependencies (libjack-jackd2-dev:armhf).  Configure
the linker for the target in cargo.

[cross-compiling rust for the
RPi](https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658)

### pi-blaster

An initial implementation interfaced with pi-blaster, but pi-blaster
didn't support the latency that is needed in audio processing.  With
10fps it can keep up, but with 60fps a delay becomes noticable after a
minute of use.

### rust library

There is a rust library that provides functionality similar to
pi-blaster.  It also needs to be run as root.
- TODO: What are the permissions that are necessary exactly?

### MIDI

I want to implement something where you can write a yaml config and
define a mapping from MIDI signals to GPIO pins.  One LED for one
color could be mapped to one GPIO, and that is then mapped to a midi
knob.

For testing, also implement a mock-gui that mocks LEDs.

[Py-Lights](https://github.com/aaron64/py-lights) allows to map midi
signals to light effects.  That requires manual light-making though.
It doesn't expose the GPIO pins as knobs directly.

Then also write it as a VST so it can be started with carla.

## Neural Processing ideas

[neural network beat
prediction](https://nlml.github.io/neural-networks/detecting-bpm-neural-networks/)

Idea: use a neural network to detect beats live.  Could be recurrent.
Use frequency spectrums as input vectors (512 floats or so).  The
input is then a sequence of these spectrums, where 512 floats relate
to 10-20ms of audio.

Create a sort of tagger, which could tag beat/no-beat, or
kick-start/kick/no-kick or something like that (like named entity
recognition).

Training data could be created from other algorithms that analyze
statically, to train an online network.

## Another Idea: Arduino LED-Shield Interfacing

I have the Arduino LED shield, I should expose that as a MIDI device.


## GUI visualization from pure MIDI

It would be interesting to think about visualizing from MIDI signals,
maybe with just a simple MIDI clock or pulse signals corresponding to
the BPM.


## FFT

how many samples for FFT?

48000Hz sample rate

lowest frequncy to detect = 100Hz

windowsize should be 480 samples with 48kHz samplerate

windowsize 1024 -> 21.3ms delay  (twice that because of double buffer)
should be more than enough

human noticable delay is 25ms  might be too close.



