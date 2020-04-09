# lumi

`lumi` is an audio to light project.

At the moment it consists of a server that runs on a Raspberry Pi and
controls GPIO attached RGB lights.  It reads input from a playstation
3 controller (dualshock six axis) on which the lights can be
controlled in an intuitive way, like an instrument.

It also supports reading audio from a microphone input and extracting
features from the audio signal to map to lights.

The goal is to create a tool that allows for expressive visualization
of sound, combining user input and automatic audio processing.


## Components

The project has multiple components which communicate with each other
over the [OSC](http://opensoundcontrol.org/) protocol, also over
network.  The central component is the server, which is running on as
Raspberry Pi and controls the actual lights.  It takes in input from
HIDs and audio processing tools.

A playstation controller can be connected, either directly to the
Raspberry Pi or to another computer from which its signals are then
sent to the server on the Raspberry Pi.  On another device audio
processing can be done and a control signal can be sent to the server,
which is then incorporated into the sound visualization as well.

Each component should send all its available information to the server
and the server then decides what influences the lights.  Otherwise the
clients would need to coordinate.

An early stage of the project included a web interface, this might be
worth considering to allow controlling the server.


## Building and Installing

### System dependencies

some stuff.

sudo apt install libusb-1.0-0-dev:armhf
sudo apt install libudev-dev:armhf libfox-1.6-dev:armhf
  
### Builing for arm

    cargo build --release --target armv7-unknown-linux-gnueabihf --bin lumi
    
### Build the Website

    cd client
    npm start build
    
Copy the contents of the build directory to `/usr/local/lib/lumi/web/`.

    
### System Dependencies

- pi-blaster


### Installation

The binary and config file needs to be in `/opt/lumi`.

#### systemd

put the following in `/etc/systemd/system/lumi.service`:

    [Unit]
    Description=The Lumi light server
    After=pi-blaster.service
    Requires=pi-blaster.service
    
    [Service]
    Type=simple
    Environment=RUST_BACKTRACE=1
    WorkingDirectory=/opt/lumi
    ExecStart=/opt/lumi/lumi
    
    [Install]
    WantedBy=multi-user.target



## Ideas

### New Mode

A new mode where both sticks are a "blob" that is moved around.  One
is red, the other is blue.  Can be a nice mix of colors.  Make the
controller very symmetrical, have one side for each blob.

A new mode that allows selecting colors for the individual lights.
Hue can again be selected with the right stick.  saturation and value
with the triggers.  The left stick indicates which stick color should
be changed.

### OSC Interface

I could implement a OSC interface to allow to control the lights over
the network.  This way it would make it easy to control the lights
from any device that supports OSC, and there are a few libraries in
different languages:
- [rosc](https://docs.rs/rosc/0.3.0/rosc/) can be used on the pi to
  provide the interface.  I can also use it to write a driver for the
  controller that runs on the pi and connects to the controller if it
  is plugged in.  I could run the same driver on a desktop PC and send
  the controller signal over network.  It needs to be paired with UDP
  for transmission.
- [JavaOSC](https://github.com/hoijui/JavaOSC) could easily be used to
  write a little java program that reads midi on windows and writes it
  to the network.

I should seperate into a seperate sub project (directory) a driver for
the controller.

I should have the functionality of the lights exposed as an OSC
interface, and I should have the controller driver take care of the
logic to turn button presses etc into OSC messages to the lights.

This way the driver can have seperate settings, which things it should
modify (i.e. which messages to send) and I can also send messages from
another software.  I.e. I just modify the color with the stick from
the controller, but another software sets the light intensity in the
rythm of the music.

----

I could make a seperate module for the controlling of the lights, that
just receives color over OSC.  for each light I could have an OSC
address, and specify for each address the pins for RGB.  This could be
fully specified in a yaml file that the module reads on startup.

In a seperate yaml file on the server I map coordinates to adresses.

### Audio Processing

I need a better introspection view for debugging, so I should refactor
the jack code so the handler just gets access to the SignalProcessor
from which it can extract the values it wants.  I can still send the
highlevel values that way, but I can also implement a more detailed
value extraction with a shared memory for debugging.

I need to visualize the whole spectrum that I'm measuring, all the
little bandpass filters.

I want to make a beat detection and I want to make a melody detection
by looking specifically at the frequencies in the mid range.

### Jack configuration in config file

I need to be able to configure the audio_in port from the config file.
I also need to be able to disable the jack module all together so
people can use it without jack and it doesn't crash.  maybe a config
option like: "audio: system:capture_1" to set the port or specify
"audio: off" to disable jack explicitly.  If the "audio" option is
missing from the config file, raise a warning.

### Light intensity normalization function

I need a function that takes a linear 0 to 1 value and somehow maps it
to values that look linear in the lights too.  The lights are very
sensitive from 0 to about 0.15 and not so much above that.
