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

To deploy on the Raspberry Pi, the software needs to be compiled for
the ARM architecture.

    cargo build --release --target armv7-unknown-linux-gnueabihf --bin lumi_server
    
    
### System Dependencies

These things need to be installed on the Raspberry Pi.

- `pi-blaster`: Is needed to set the pins, controlling the LEDs.
  pi-blaster can be found on
  [github](https://github.com/sarfata/pi-blaster)
- `libjack0`: This dependency is required because lumi is compiled
  with jack support.  You don't actually need to install jack if you
  don't want to do audio processing, but the library still needs to be
  there.  Can be installed with `sudo apt install libjack0`.


### Installation

The binary and config file (`conf.yaml`) needs to be in `/opt/lumi`.

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


## Configuration

The software is configurated in a `conf.yaml` file that should be in
the working directory.  The config file could look like this:

    audio-in: system:capture_1
    pi-blaster: /dev/pi-blaster
    lights:
      Top:
        r: 14
        g: 15
        b: 18
      Bottom:
        r: 23
        g: 25
        b: 24
      Left:
        r: 17
        g: 27
        b: 22
      Right:
        r: 10
        g: 9
        b: 11

`audio-in` is a `device:port` entry that specifies which jack port to
read from.  It can also be set to `off` to disable reading audio input.
Do this if jack is not setup on the device.

`pi-blaster` refers to the path of the pi-blaster device path.

The numbers in the lights refer to GPIO pin numbers.  The number of a
pin can be found [here](https://pinout.xyz/).
