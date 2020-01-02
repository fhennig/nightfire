# lumi

So far I implemented reading a YAML config and setting lights with
pi-blaster.

Next steps:
- Add client code to the repo
- Make Iron serve the js code.
  
## Builing for arm

    cargo build --release --target armv7-unknown-linux-gnueabihf --bin lumi
    
## System Dependencies
- pi-blaster
- nginx
    
## Installation

Put binary in /usr/local/bin/lumi
Put web files in /usr/local/lib/lumi/web/*
Put a systemd unit file in /etc/systemd/system/lumi.service
Put an nginx file in the correct place
Put the config file in /etc/lumi/conf.yaml

## GraphQL API

I want different "modes" and each mode has settings

Modes:
- Set all lights at once to one color
- Set each light individually
- a "fire" mode that has glowing red yellow and orange tones
  - optionally supports different colors too
- a bpm blink mode

The software could load with each mode with default parameters.
Then there is a mutation to change the active mode.
Each mode also has a mutation to change the parameters.

In the UI there is a control to change the active mode, and each mode
has it's own controls as well to select the parameters.

Every mode has a couple of functions:
- initialize: sets the inital parameter values
- activate: should set the pins according to the parameters.  Could
  optionally take parameters as well.  There could be specific
  behaviour for certain state transitions.
- deactivate: does something too
- teardown?

The question is, if I have some lights setup, and then switch to a
mode, should it somehow keep my lights or should it not?  I.e. how
much shared state is there?  If I have four individual colors set and
then switch to "single color mode", what happens?

